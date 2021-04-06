// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
pragma abicoder v2;

import {TypedMemView} from "./libraries/TypedMemView.sol";
import {ViewCKB} from "./libraries/ViewCKB.sol";
import {ICKBSpv} from "./interfaces/ICKBSpv.sol";
import {MultisigUtils} from "./libraries/MultisigUtils.sol";

// tools below just for test, they will be removed before production ready
//import "./test/console.sol";

contract CKBChain is ICKBSpv {
    using TypedMemView for bytes;
    using TypedMemView for bytes29;
    using ViewCKB for bytes29;

    bool public initialized;
    uint64 public latestBlockNumber;
    uint64 public initBlockNumber;

    // refer to https://github.com/ethereum/EIPs/blob/master/EIPS/eip-712.md
    uint public constant SIGNATURE_SIZE = 65;
    uint public constant VALIDATORS_SIZE_LIMIT = 20;
    string public constant NAME_712 = "Force Bridge CKBChain";

    /* solhint-disable var-name-mixedcase */
    // Cache the domain separator as an immutable value, but also store the chain id that it corresponds to, in order to
    // invalidate the cached domain separator if the chain id changes.
    bytes32 private _CACHED_DOMAIN_SEPARATOR;
    uint256 private _CACHED_CHAIN_ID;

    bytes32 private _HASHED_NAME;
    bytes32 private _HASHED_VERSION;
    bytes32 private _TYPE_HASH;

    // if the number of verified signatures has reached `multisigThreshold_`, validators approve the tx
    uint public multisigThreshold_;
    address[] validators_;

    // CKBChainV3-----------------------------
    // ADD_HISTORY_TX_ROOT_TYPEHASH = keccak256("AddHistoryTxRoot(uint64 startBlockNumber, uint64 endBlockNumber, bytes32 historyTxRoot)");
    bytes32 public constant ADD_HISTORY_TX_ROOT_TYPEHASH = 0x0eeee1be1069b2c737b19f6c3510ceeed099af9ee1f5985109f117ce0524ca10;
    // SET_NEW_VALIDATORS_TYPEHASH = keccak256("SetNewValidators(address[] validators, uint256 multisigThreshold)");
    bytes32 public constant SET_NEW_VALIDATORS_TYPEHASH = 0x2dbb88776b74460392119c32a74db39f2a5cd7dd954f3934412fcadb945fdab3;
    bytes32 public historyTxRoot;
    mapping(bytes32 => bytes32) verifiedTxRoots;

    event HistoryTxRootAdded(
        uint64 indexed startBlockNumber,
        uint64 indexed endBlockNumber,
        bytes32 HistoryTxRoot
    );

    /**
     * @notice  if addr is not one of validators_, return validators_.length
     * @return  index of addr in validators_
     */
    function _getIndexOfValidators(address user) internal view returns (uint) {
        for (uint i = 0; i < validators_.length; i++) {
            if (validators_[i] == user) {
                return i;
            }
        }
        return validators_.length;
    }


    /**
     * @notice             @dev signatures are a multiple of 65 bytes and are densely packed.
     * @param signatures   The signatures bytes array
     */
    function validatorsApprove(bytes32 msgHash, bytes memory signatures) public view {
        require(signatures.length % SIGNATURE_SIZE == 0, "invalid signatures");

        // 1. check length of signature
        uint length = signatures.length / SIGNATURE_SIZE;
        require(length >= multisigThreshold_, "length of signatures must greater than threshold");

        // 3. check number of verified signatures >= threshold
        uint verifiedNum = 0;
        uint i = 0;

        uint8 v;
        bytes32 r;
        bytes32 s;
        address recoveredAddress;
        // set indexVisited[ index of recoveredAddress in validators_ ] = true
        bool[] memory validatorIndexVisited = new bool[](validators_.length);
        uint validatorIndex;
        while (i < length) {
            (v, r, s) = MultisigUtils.parseSignature(signatures, i);
            i++;

            recoveredAddress = ecrecover(msgHash, v, r, s);
            require(recoveredAddress != address(0), "invalid signature");

            // get index of recoveredAddress in validators_
            validatorIndex = _getIndexOfValidators(recoveredAddress);

            // recoveredAddress is not validator or has been visited
            if (validatorIndex >= validators_.length || validatorIndexVisited[validatorIndex]) {
                continue;
            }

            // recoveredAddress verified
            validatorIndexVisited[validatorIndex] = true;
            verifiedNum++;
            if (verifiedNum >= multisigThreshold_) {
                return;
            }
        }

        require(verifiedNum >= multisigThreshold_, "signatures not verified");
    }

    function initialize(
        address[] memory validators,
        uint multisigThreshold
    ) public {
        require(!initialized, "Contract instance has already been initialized");
        initialized = true;

        // set DOMAIN_SEPARATOR
        bytes32 hashedName = keccak256(bytes(NAME_712));
        bytes32 hashedVersion = keccak256(bytes("1"));
        bytes32 typeHash = keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
        _HASHED_NAME = hashedName;
        _HASHED_VERSION = hashedVersion;
        _CACHED_CHAIN_ID = _getChainId();
        _CACHED_DOMAIN_SEPARATOR = _buildDomainSeparator(typeHash, hashedName, hashedVersion);
        _TYPE_HASH = typeHash;

        // set validators
        _setNewValidators(validators, multisigThreshold);
    }

    function setNewValidators(
        address[] calldata validators,
        uint256 multisigThreshold,
        bytes calldata signatures
    ) external {
        // 1. calc msgHash
        bytes32 msgHash = keccak256(
            abi.encodePacked(
                "\x19\x01", // solium-disable-line
                _domainSeparatorV4(),
                keccak256(abi.encode(SET_NEW_VALIDATORS_TYPEHASH, validators, multisigThreshold))
            )
        );

        // 2. validatorsApprove
        validatorsApprove(msgHash, signatures);

        // 3. _setNewValidators
        _setNewValidators(validators, multisigThreshold);
    }

    // CKBChainV3-----------------------------
    function addHistoryTxRoot(uint64 _initBlockNumber, uint64 _latestBlockNumber, bytes32 _historyTxRoot, bytes calldata signatures)
    external
    {
        require(_latestBlockNumber > latestBlockNumber, "latestBlockNumber should be strictly incremental");

        // 1. calc msgHash
        bytes32 msgHash = keccak256(
            abi.encodePacked(
                "\x19\x01", // solium-disable-line
                _domainSeparatorV4(),
                keccak256(abi.encode(ADD_HISTORY_TX_ROOT_TYPEHASH, _initBlockNumber, _latestBlockNumber, _historyTxRoot))
            )
        );

        // 2. validatorsApprove
        validatorsApprove(msgHash, signatures);

        initBlockNumber = _initBlockNumber;
        latestBlockNumber = _latestBlockNumber;
        historyTxRoot = _historyTxRoot;

        // 3. event
        emit HistoryTxRootAdded(_initBlockNumber, _latestBlockNumber, _historyTxRoot);
    }

    function getHistoryTxRootInfo() override external view returns (uint64, uint64, bytes32) {
        return (initBlockNumber, latestBlockNumber, historyTxRoot);
    }

    function DOMAIN_SEPARATOR() external view returns (bytes32) {
        return _domainSeparatorV4();
    }

    function _setNewValidators(
        address[] memory validators,
        uint multisigThreshold
    ) internal {
        // set validators
        require(validators.length <= VALIDATORS_SIZE_LIMIT, "number of validators exceeds the limit");
        require(multisigThreshold <= validators.length, "invalid multisigThreshold");
        validators_ = validators;
        multisigThreshold_ = multisigThreshold;
    }

    function _buildDomainSeparator(bytes32 typeHash, bytes32 name, bytes32 version) private view returns (bytes32) {
        return keccak256(
            abi.encode(
                typeHash,
                name,
                version,
                _getChainId(),
                address(this)
            )
        );
    }

    /**
     * @dev Returns the domain separator for the current chain.
     */
    function _domainSeparatorV4() internal view virtual returns (bytes32) {
        if (_getChainId() == _CACHED_CHAIN_ID) {
            return _CACHED_DOMAIN_SEPARATOR;
        } else {
            return _buildDomainSeparator(_TYPE_HASH, _HASHED_NAME, _HASHED_VERSION);
        }
    }

    function _getChainId() private view returns (uint256 chainId) {
        this; // silence state mutability warning without generating bytecode - see https://github.com/ethereum/solidity/issues/2691
        // solhint-disable-next-line no-inline-assembly
        assembly {
            chainId := chainid()
        }
    }
}
