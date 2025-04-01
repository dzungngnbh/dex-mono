package main

import (
	"fmt"
	"os"
	"os/exec"
	"strings"
)

// TODO: tar folder and extract remotely

// Deployer is responsible for handling the deployment process
type Deployer struct {
	serverIP       string
	serviceName    string
	systemdService string
	r2URL          string
	binPath        string
	binURL         string
	envrcURL       string
}

// NewDeployer creates and initializes a new Deployer instance
func NewDeployer() (*Deployer, error) {
	serverIP := os.Getenv("SERVER_IP")
	serviceName := os.Getenv("SERVICE_NAME")
	r2URL := os.Getenv("R2_URL")

	if serverIP == "" {
		return nil, fmt.Errorf("SERVER_IP environment variable is missing")
	}
	if serviceName == "" {
		serviceName = "web_app" // Default value if not provided
	}
	if r2URL == "" {
		return nil, fmt.Errorf("R2_URL environment variable is missing")
	}

	binPath := fmt.Sprintf("%s/%s", serviceName, serviceName)

	return &Deployer{
		serverIP:       serverIP,
		serviceName:    serviceName,
		systemdService: fmt.Sprintf("%s.service", serviceName),
		r2URL:          r2URL,
		binPath:        binPath,
		binURL:         fmt.Sprintf("%s/%s", r2URL, binPath),
		envrcURL:       fmt.Sprintf("%s/%s/.envrc", r2URL, serviceName),
	}, nil
}

// RunCommand executes a shell command and returns an error if it fails
func (d *Deployer) RunCommand(command string) error {
	cmd := exec.Command("bash", "-c", command)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	return cmd.Run()
}

// GetDirName extracts the directory name from a path
func (d *Deployer) GetDirName() string {
	parts := strings.Split(d.binPath, "/")
	return strings.Join(parts[:len(parts)-1], "/")
}

// CopyEnvrc copies the .envrc file if it exists
func (d *Deployer) CopyEnvrc() error {
	// Check if .envrc file exists
	if _, err := os.Stat(".envrc"); err == nil {
		command := fmt.Sprintf("rclone copy .envrc r2:deployments/%s", d.GetDirName())
		if err := d.RunCommand(command); err != nil {
			return fmt.Errorf("failed to copy .envrc: %v", err)
		}
		fmt.Printf(".envrc is uploaded.\n")
	} else {
		fmt.Println(".envrc file not found, skipping copy.")
	}
	return nil
}

// BuildAndUpload builds the project and uploads artifacts
func (d *Deployer) BuildAndUpload() error {
	if err := d.RunCommand("cargo build --release"); err != nil {
		return fmt.Errorf("build failed: %v", err)
	}

	command := fmt.Sprintf("rclone copy ../../target/release/%s r2:deployments/%s",
		d.serviceName, d.GetDirName())
	if err := d.RunCommand(command); err != nil {
		return fmt.Errorf("failed to upload binary: %v", err)
	}

	fmt.Printf("%s is uploaded to %s\n", d.serviceName, d.binURL)
	return nil
}

// ExecuteSSHCommand runs deployment commands on the remote server
func (d *Deployer) ExecuteSSHCommand() error {
	// SSH command with heredoc to create service file if needed
	sshCommand := fmt.Sprintf(`
		mkdir -p /root/apps/%s;
		cd /root/apps/%s;
		curl %s -o .envrc; # Download the .envrc file
		direnv allow;
		# Check if the systemd service file exists, if not create it
		if [ ! -f /etc/systemd/system/%s ]; then
			# Create the service file
			cat <<EOF > /etc/systemd/system/%s
[Unit]
Description=Trading webapp
After=network.target
[Service]
Type=simple
User=root
WorkingDirectory=/root/apps/%s/
ExecStart=direnv exec . ./%s
[Install]
WantedBy=multi-user.target
EOF
			# Reload systemd and enable the service
			systemctl daemon-reload
			systemctl enable %s;
			echo "Service is created and enabled.";
		else
			echo "Service already exists, skipping creation.";
		fi

		systemctl stop %s || true;
		curl %s -o %s;
		chmod +x %s;
		systemctl start %s;
		sleep 1s;
		systemctl status %s;
	`, d.GetDirName(), d.GetDirName(), d.envrcURL, d.systemdService, d.systemdService,
		d.GetDirName(), d.serviceName, d.systemdService, d.systemdService,
		d.binURL, d.serviceName, d.serviceName, d.systemdService, d.systemdService)

	command := fmt.Sprintf("ssh root@%s '%s'", d.serverIP, sshCommand)
	return d.RunCommand(command)
}

// Deploy performs the complete deployment process
func (d *Deployer) Deploy() error {
	fmt.Println("Building and uploading artifacts...")
	if err := d.BuildAndUpload(); err != nil {
		return err
	}

	if err := d.CopyEnvrc(); err != nil {
		return err
	}

	fmt.Println("Deploying...")
	if err := d.ExecuteSSHCommand(); err != nil {
		return fmt.Errorf("deployment failed: %v", err)
	}

	fmt.Println("Deployment successful.")
	return nil
}

func main() {
	deployer, err := NewDeployer()
	if err != nil {
		fmt.Printf("Initialization error: %v\n", err)
		os.Exit(1)
	}

	if err := deployer.Deploy(); err != nil {
		fmt.Printf("Error: %v\n", err)
		os.Exit(1)
	}
}
