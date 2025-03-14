import { devices, PlaywrightTestConfig } from "@playwright/test"
import path from "path"
import * as net from "net"

// function checkPortStatus(port: number, host: string = 'localhost'): Promise<boolean> {
//   return new Promise((resolve) => {
//     const socket = new net.Socket();
//
//     socket.on('connect', () => {
//       // The port is open and a connection was successful
//       socket.destroy();
//       resolve(true);
//     });
//
//     socket.on('error', () => {
//       // The port is closed or there was an error
//       resolve(false);
//     });
//
//     socket.connect(port, host);
//   });
// }

// Use process.env.PORT by default and fallback to port 3000
const PORT = process.env.PORT || 3000

// Set webServer.url and use.baseURL with the location of the WebServer respecting the correct set port
const baseURL = `https://localhost:${PORT}`

// Reference: https://playwright.dev/docs/test-configuration
const config: PlaywrightTestConfig = {
  // Timeout per test
  timeout: 30 * 1000,
  // Test directory
  testDir: path.join(__dirname, "e2etests"),
  // If a test fails, retry it additional 2 times
  retries: 1,
  // Artifacts folder where screenshots, videos, and traces are stored.
  outputDir: "test-results/",

  // Run your local dev server before starting the tests:
  // https://playwright.dev/docs/test-advanced#launching-a-development-web-server-during-the-tests
  // webServer: {
  //   command: "pnpm next dev",
  //   url: baseURL,
  //   timeout: 120 * 1000,
  //   reuseExistingServer: !process.env.CI,
  // },

  use: {
    // Use baseURL so to make navigations relative.
    // More information: https://playwright.dev/docs/api/class-testoptions#test-options-base-url
    baseURL,

    // Retry a test if its failing with enabled tracing. This allows you to analyse the DOM, console logs, network traffic etc.
    // More information: https://playwright.dev/docs/trace-viewer
    trace: "retry-with-trace",
    // All available context options: https://playwright.dev/docs/api/class-browser#browser-new-context
    // contextOptions: {
    //   ignoreHTTPSErrors: true,
    // },
  },

  projects: [
    {
      name: "Desktop Chrome",
      use: {
        ...devices["Desktop Chrome"],
        headless: false,
        launchOptions: {
          executablePath:
            "C:\\Users\\Administrator\\AppData\\Local\\Google\\Chrome SxS\\Application\\chrome.exe",
        },
      },
    },
    // {
    //   name: 'Desktop Firefox',
    //   use: {
    //     ...devices['Desktop Firefox'],
    //   },
    // },
    // {
    //   name: 'Desktop Safari',
    //   use: {
    //     ...devices['Desktop Safari'],
    //   },
    // },
    // Test against mobile viewports.
  ],
}
export default config
