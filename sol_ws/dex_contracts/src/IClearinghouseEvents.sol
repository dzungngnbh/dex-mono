// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

interface IClearinghouseEvents {
    event UpdateCollateral(
        uint32 productId,
        bytes32 indexed subaccount,
        int128 amount
    );
}
