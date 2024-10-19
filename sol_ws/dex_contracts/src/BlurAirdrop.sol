// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";

import "./interfaces/IERC20Base.sol";
import "./libraries/ERC20Helper.sol";

import {IBlurAirdrop} from "./interfaces/IBlurAirdrop.sol";

contract BlurAirdrop is IBlurAirdrop, OwnableUpgradeable, UUPSUpgradeable {
    address token;
    uint32 pastWeeks;

    mapping(uint32 => bytes32) weekMerkleRoots; // week => merkleRoot
    mapping(uint32 => mapping(address => uint256)) claimed; // claimed[week][account] = amount

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    ///@dev required by the OZ UUPS module
    function _authorizeUpgrade(address) internal override onlyOwner {}

    function initialize(address _token)  initializer public {
        require(_token != address(0), "invalid token address");

        __Ownable_init(msg.sender);
        __UUPSUpgradeable_init();

        token = _token;
    }

    function registerMerkleRoot(uint32 week, bytes32 merkleRoot) external onlyOwner
    {
        pastWeeks += 1;
        require(week == pastWeeks, "Invalid week provided.");
        weekMerkleRoots[week] = merkleRoot;
    }

    function _verifyProof(
        uint32 week,
        address sender,
        uint256 totalAmount,
        bytes32[] calldata proof
    ) internal {
        require(weekMerkleRoots[week] != bytes32(0), "invalid week");
        require(claimed[week][msg.sender] == 0, "already claimed");

        bytes32 leaf = keccak256(
            bytes.concat(keccak256(abi.encode(sender, totalAmount)))
        );
        bool isValidLeaf = MerkleProof.verify(proof, weekMerkleRoots[week], leaf);
        require(isValidLeaf, "invalid proof");
        claimed[week][msg.sender] = totalAmount;
    }

    function _claim(ClaimProof calldata claimProof) internal {
        uint32 week = claimProof.week;

        _verifyProof(week, msg.sender, claimProof.totalAmount, claimProof.proof);
        ERC20Helper.safeTransfer(IERC20Base(token), msg.sender, claimProof.totalAmount);
        emit ClaimAirdrop(msg.sender, week, claimProof.totalAmount);
    }

    function claim(ClaimProof[] calldata claimProofs) public {
        for (uint32 i = 0; i < claimProofs.length; i++) {
            _claim(claimProofs[i]);
        }
    }

    function getClaimed(address account) external view returns (uint256[] memory) {
        uint256[] memory result = new uint256[](pastWeeks + 1);
        for (uint32 week = 1; week <= pastWeeks; week++) {
            result[week] = claimed[week][account];
        }
        return result;
    }
}
