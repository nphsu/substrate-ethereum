pragma solidity ^0.5.8;

import 'openzeppelin-solidity/contracts/token/ERC20/SafeERC20.sol';

contract DAIBridge {
    IERC20 private token;

    event RelayMessage();

    constructor (IERC20 _token) public {
        token = _token;
    }

    function setTransfer() public {
        // TODO: validate
        // token.transferFrom(msg.sender, address(this), amount);
        // TODO: event
        emit RelayMessage();
    }
}