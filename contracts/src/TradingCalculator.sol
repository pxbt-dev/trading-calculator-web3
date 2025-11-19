// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract TradingCalculator {
    
    event PositionCalculated(
        address indexed user,
        uint256 unitsToBuy,
        uint256 totalPositionSize,
        string riskRewardRatio
    );
    
    function calculatePosition(
        uint256 accountSize,
        uint256 riskDollars, 
        uint256 entryPrice,
        uint256 stopLoss,
        uint256 targetPrice
    ) public pure returns (
        uint256 unitsToBuy,
        uint256 totalPositionSize,
        uint256 riskPerShare,
        uint256 riskPercentage,
        string memory riskRewardRatio
    ) {
        require(accountSize > 0, "Account size must be positive");
        require(riskDollars > 0, "Risk amount must be positive");
        require(entryPrice > 0, "Entry price must be positive");
        require(stopLoss > 0, "Stop loss must be positive");
        require(riskDollars <= accountSize, "Risk cannot exceed account size");
        
        // Calculate risk per share
        riskPerShare = entryPrice > stopLoss ? entryPrice - stopLoss : stopLoss - entryPrice;
        require(riskPerShare > 0, "Entry and stop loss too close");
            
        // Calculate units to buy
        unitsToBuy = riskDollars / riskPerShare;
        totalPositionSize = unitsToBuy * entryPrice;
        
        // Risk percentage (1.00% = 100)
        riskPercentage = (riskDollars * 10000) / accountSize;
        
        // Risk/Reward ratio
        riskRewardRatio = "N/A";
        if (targetPrice > 0 && riskPerShare > 0) {
            uint256 reward = targetPrice > entryPrice ? targetPrice - entryPrice : entryPrice - targetPrice;
            uint256 ratio = reward / riskPerShare;
            riskRewardRatio = string(abi.encodePacked("1:", uintToString(ratio)));
        }
    }
    
    function uintToString(uint256 _i) internal pure returns (string memory) {
        if (_i == 0) return "0";
        uint256 j = _i;
        uint256 len;
        while (j != 0) {
            len++;
            j /= 10;
        }
        bytes memory bstr = new bytes(len);
        uint256 k = len;
        while (_i != 0) {
            unchecked {
                k = k - 1;
            }
            uint8 temp = (48 + uint8(_i - _i / 10 * 10));
            bytes1 b1 = bytes1(temp);
            bstr[k] = b1;
            _i /= 10;
        }
        return string(bstr);
    }
    
    function ping() public pure returns (string memory) {
        return "Trading Calculator is working!";
    }
}
