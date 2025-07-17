// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

contract HorseStore {
    uint256 numberOfHorse;

    function updateHorseNumber(uint256 newNumberOfHorses) external {
        numberOfHorse = newNumberOfHorses;
    }

    function readNumberOfHorses() external view returns (uint256) {
        return numberOfHorse;
    }
}
