pragma solidity 0.4.22;

contract MisusedBoolean {

    function isEven(uint256 num) internal returns(bool) {
        return num % 2 == 0;
    }

    function misuse(uint256 num) external returns(uint256) {
        if (isEven(num) || true) {
            return num * num;
        }
        return 0;
    }

    function misuse2(uint256 num) external returns(uint256) {
        if (isEven(num) && false) {
            return num * num;
        }
        return 0;
    }

    function misuse3(uint256 num) external returns(uint256) {
        if (false && isEven(num)) {
            return num * num;
        }
        return 0;
    }

    function misuse4(uint256 num) external returns(uint256) {
        if (true || isEven(num)) {
            return num * num;
        }
        return 0;
    }

    function misuse5(uint256 num) external pure returns(uint256) {
        if (true) {
            return num * num;
        }
        return 0;
    }

    function misuse6(uint256 num) external pure returns(uint256) {
        if (false) {
            return num * num;
        }
        return 0;
    }

}