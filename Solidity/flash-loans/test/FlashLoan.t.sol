// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {FlashLoan} from "../src/FlashLoan.sol";
import "aave-v3-core/contracts/interfaces/IPoolAddressesProvider.sol";
import "@openzeppelin/contracts/interfaces/IERC20.sol";

contract FlashLoanTest is Test {
    //Ethereum Mainnet DAI Contract Address
    address constant DAI = 0x6B175474E89094C44Da98b954EedeAC495271d0F;

    // Mainnet Pool contract address
    address constant POOL_ADDRESS_PROVIDER =
        0x2f39d218133AFaB8F2B819B1066c7E434Ad94E9e;

    //vm.envString is provided by Foundry so that we can read our .env file and get values from there
    string MAINNET_RPC_URL = vm.envString("MAINNET_RPC_URL");
    uint256 mainnetFork;
    IERC20 public token;

    FlashLoan public flashLoan;

    function setUp() public {
        // vm is a variable included in the forge standard library that is used to manipulate the execution environment of our tests
        // create a fork of Ethereum mainnet using the specified RPC URL and store its id in mainnetFork
        mainnetFork = vm.createFork(MAINNET_RPC_URL);
        //select the fork thus obtained, using its id
        vm.selectFork(mainnetFork);
        //deploy FlashLoanExample to the created fork with POOL_ADDRESS_PROVIDER as its constructor argument
        flashLoan = new FlashLoan(
            IPoolAddressesProvider(POOL_ADDRESS_PROVIDER)
        );
        //fetch the DAI contract
        token = IERC20(DAI);
    }

    function testTakeAndReturnLoan() public {
        // Get 2000 DAI in our contract by using deal
        // deal is a cheatcode that lets us arbitrarily set the balance of any address and works with most ERC-20 tokens
        uint BALANCE_AMOUNT_DAI = 2000 ether;
        deal(DAI, address(flashLoan), BALANCE_AMOUNT_DAI);

        // Request and execute a flash loan of 10,000 DAI from Aave
        flashLoan.createFlashLoan(DAI, 10000);

        // By this point, we should have executed the flash loan and paid back (10,000 + premium) DAI to Aave
        // Let's check our contract's remaining DAI balance to see how much it has left
        uint remainingBalance = token.balanceOf(address(flashLoan));

        // Our remaining balance should be <2000 DAI we originally had, because we had to pay the premium
        //asserLt => assert strictly less than
        assertLt(remainingBalance, BALANCE_AMOUNT_DAI);
    }
}
