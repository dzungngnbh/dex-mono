// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

// TODO: add address to current list
interface ISanctionsList {
    function isSanctioned(address addr) external view returns (bool);
}