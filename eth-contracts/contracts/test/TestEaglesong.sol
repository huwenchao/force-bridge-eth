pragma solidity ^0.7.4;
import {Eaglesong} from "../Eaglesong.sol";

contract TestEaglesong {
    address public songAddr;

    constructor(address _songAddr) {
        songAddr = _songAddr;
    }

    function ckbEaglesong(bytes memory data) public returns (bytes32 result) {
        bytes memory payload = abi.encodePacked(
            hex"aa6d7de4",
            data
        );

        (, bytes memory returnData) = songAddr.staticcall(payload);

        assembly {
        // solium-disable-previous-line security/no-inline-assembly
            result := mload(add(returnData, 0x20))
        }
    }
}
