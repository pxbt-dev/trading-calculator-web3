// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../src/TradingCalculator.sol";

contract TradingCalculatorTest is Test {
    TradingCalculator calculator;
    
    function setUp() public {
        calculator = new TradingCalculator();
    }
    
    function testPing() public view {
        string memory result = calculator.ping();
        assertEq(result, "Trading Calculator is working!");
    }
    
    function testCalculatePosition() public view {
        (
            uint256 unitsToBuy,
            uint256 totalPositionSize,
            uint256 riskPerShare,
            uint256 riskPercentage,
            string memory riskRewardRatio
        ) = calculator.calculatePosition(10000, 100, 50, 48, 55);
        
        // Should calculate correct values
        assertEq(riskPerShare, 2); // 50 - 48 = 2
        assertEq(unitsToBuy, 50); // 100 / 2 = 50
        assertEq(totalPositionSize, 2500); // 50 * 50 = 2500
        assertEq(riskPercentage, 100); // (100 * 10000) / 10000 = 100 (1%)
        assertEq(riskRewardRatio, "1:2"); // (55-50=5) / (50-48=2) = 2, but integer division gives 2
    }
    
    function testNoTargetPrice() public view {
        (
            uint256 unitsToBuy,
            uint256 totalPositionSize,
            uint256 riskPerShare,
            uint256 riskPercentage,
            string memory riskRewardRatio
        ) = calculator.calculatePosition(10000, 100, 50, 48, 0);
        
        assertEq(riskPerShare, 2);
        assertEq(unitsToBuy, 50);
        assertEq(riskRewardRatio, "N/A"); // No target price
    }
    
    function testValidation() public {
        // Should revert if risk exceeds account size
        vm.expectRevert("Risk cannot exceed account size");
        calculator.calculatePosition(100, 200, 50, 48, 55);
        
        // Should revert if entry equals stop loss
        vm.expectRevert("Entry and stop loss too close");
        calculator.calculatePosition(10000, 100, 50, 50, 55);
    }
}
