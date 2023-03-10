export type DebridgeInvokeExample = {
  version: "0.0.0";
  name: "debridge_invoke_example";
  instructions: [
    {
      name: "sendViaDebridge";
      docs: [
        "Debridge protocol allows transfer liquidity from Solana to other supported chains",
        "To send some token to other supported chain use [`debridge_solana_sdk::sending::invoke_debridge_send`]",
        "",
        "To check if the network is supported use [`debridge_solana_sdk::sending::is_chain_supported`]",
      ];
      accounts: [];
      args: [
        {
          name: "amount";
          type: "u64";
        },
        {
          name: "targetChainId";
          type: {
            array: ["u8", 32];
          };
        },
        {
          name: "receiver";
          type: "bytes";
        },
        {
          name: "isUseAssetFee";
          type: "bool";
        },
      ];
    },
    {
      name: "sendViaDebridgeWithNativeFixedFee";
      docs: [
        "Debridge protocol takes fix fee and transfer fee while sending liquidity.",
        "The fix fee by default is taken in native solana tokens.",
        "The default native fix fee amount is set in state account but it can set custom native",
        "fix amount for a specific chain in chain support info account.",
        "",
        "To get default native fix fee amount use [`debridge_solana_sdk::sending::get_default_native_fix_fee`]",
        "",
        "To get native fix fee amount for specific chain use [`debridge_solana_sdk::sending::get_chain_native_fix_fee`]",
        "",
        "To use native fix fee set [`debridge_solana_sdk::sending::SendIx`] `is_use_asset_fee` field to `false`",
      ];
      accounts: [];
      args: [
        {
          name: "amount";
          type: "u64";
        },
        {
          name: "targetChainId";
          type: {
            array: ["u8", 32];
          };
        },
        {
          name: "receiver";
          type: "bytes";
        },
      ];
    },
    {
      name: "sendViaDebridgeWithAssetFixedFee";
      docs: [
        "Debridge protocol takes fix fee and transfer fee while sending liquidity.",
        "The fix fee by default is taken in native solana tokens.",
        "But when transferring some tokens to certain networks, it is possible to pay in transferred tokens.",
        "It's called `asset_fix_fee`.",
        "",
        "To known `asset_fee` is available use [`debridge_solana_sdk::sending::is_asset_fee_available`]",
        "",
        "To get asset fix fee amount for specific chain use [`debridge_solana_sdk::sending::try_get_chain_asset_fix_fee`]",
        "",
        "To use asset fix fee set [`debridge_solana_sdk::sending::SendIx`] `is_use_asset_fee` field to `true`",
      ];
      accounts: [];
      args: [
        {
          name: "amount";
          type: "u64";
        },
        {
          name: "targetChainId";
          type: {
            array: ["u8", 32];
          };
        },
        {
          name: "receiver";
          type: "bytes";
        },
      ];
    },
    {
      name: "sendViaDebridgeWithExactAmount";
      docs: [
        "Debridge protocol takes fix fee and transfer fee while sending liquidity.",
        "If needed to get exact amount tokens in target chain, all fees will need to be added to sending amount.",
        "",
        "There are three types of fees in Debridge protocol: fixed fee, transfer fee, execution fee.",
        "",
        "Fixed fee is fixed amount for any send instruction. It's named asset fixed fee. The amount depends on target chain.",
        "To get asset fix fee amount for specific chain use [`debridge_solana_sdk::sending::try_get_chain_asset_fix_fee`]",
        "For some token fixed fee can be paid with sent tokens. In this case, you need to include this asset fixed fee in the final amount.",
        "",
        "Transfer fee is taken as part of sent tokens. To get the bps of transfer fee use [`debridge_solana_sdk::sending::get_transfer_fee`]",
        "To add transfer fee to current amount use [`debridge_solana_sdk::sending::add_transfer_fee`]",
        "",
        "Execution fee is reward for execute claim instruction in target chain. It can be zero if you want to run the instruction yourself.",
        "The recommended execution fee can be obtained using debridge sdk.",
        "",
        "To add to exact amount all fees charged during the send use [`debridge_solana_sdk::sending::add_all_fees`]",
      ];
      accounts: [];
      args: [
        {
          name: "exactAmount";
          type: "u64";
        },
        {
          name: "targetChainId";
          type: {
            array: ["u8", 32];
          };
        },
        {
          name: "receiver";
          type: "bytes";
        },
        {
          name: "executionFee";
          type: "u64";
        },
        {
          name: "isUseAssetFee";
          type: "bool";
        },
      ];
    },
    {
      name: "sendViaDebridgeWithExecutionFee";
      docs: [
        "Debridge protocol allows to anyone execute claim transaction in target chain. It allow to create",
        "fluent user experience when user only send tokens to other chain and automatically receives in another.",
        "",
        "User adds execution fee as reward for execution his claim transaction in target chain.",
        "The recommended execution fee can be obtained using debridge sdk.",
      ];
      accounts: [];
      args: [
        {
          name: "amount";
          type: "u64";
        },
        {
          name: "targetChainId";
          type: {
            array: ["u8", 32];
          };
        },
        {
          name: "receiver";
          type: "bytes";
        },
        {
          name: "executionFee";
          type: "u64";
        },
      ];
    },
    {
      name: "sendViaDebridgeWithExternalCall";
      docs: [
        "Debridge protocol allows not only to send tokens to another network,",
        "but also to use them to call any smart contract.",
        "",
        "Used `external_call` for this. For evm-like network it will be address of smart contract function and function's arguments",
        "packed in byte vector.",
        "",
        "To use external call function needed to initialize external call storage with",
        "[`debridge_solana_sdk::sending::invoke_init_external_call`] function and create `submission_params`",
        "with [`debridge_solana_sdk::sending::SendSubmissionParamsInput::with_external_call`] function.",
        "Besides external call needed to provide `fallback_address`. The `fallback_address' will be used",
        "if external call fails. On this address token received in target chain will transfer.",
        "",
        "A `execution_fee` is reward reward that will received for execution claim transaction in",
        "target chain. It can be set zero if external call will be claimed by yourself.",
      ];
      accounts: [];
      args: [
        {
          name: "amount";
          type: "u64";
        },
        {
          name: "targetChainId";
          type: {
            array: ["u8", 32];
          };
        },
        {
          name: "receiver";
          type: "bytes";
        },
        {
          name: "executionFee";
          type: "u64";
        },
        {
          name: "fallbackAddress";
          type: "bytes";
        },
        {
          name: "reservedFlag";
          type: {
            array: ["u8", 32];
          };
        },
        {
          name: "externalCall";
          type: "bytes";
        },
      ];
    },
    {
      name: "sendMessageViaDebridge";
      docs: [
        "deBridge protocol allows calling any smart contract in target chain without sending any tokens.",
        "You have to pay only a transfer fee for sending an execution fee to another chain.",
        "If you claim by yourself, set execution fee to zero, you don’t need to pay transfer fee at all.",
        "Only fixed fee will be taken.",
        "",
        "Used `external_call` for this. For evm-like network it will be address of smart contract",
        "function and function's arguments packed in byte vector.",
        "",
        "To send message with external call function use [`debridge_solana_sdk::sending::invoke_send_message`]",
        "function. This function will create external call storage, calculate transfer fee for",
        "transferring execution fee and send the message to target chain.",
        "Besides external call needed to provide `fallback_address`. The `fallback_address' will be used",
        "if external call fails. On this address token received in target chain will transfer.",
        "",
        "A `execution_fee` is reward reward that will received for execution claim transaction in",
        "target chain. It can be set zero if external call will be claimed by yourself.",
      ];
      accounts: [];
      args: [
        {
          name: "targetChainId";
          type: {
            array: ["u8", 32];
          };
        },
        {
          name: "receiver";
          type: "bytes";
        },
        {
          name: "executionFee";
          type: "u64";
        },
        {
          name: "fallbackAddress";
          type: "bytes";
        },
        {
          name: "message";
          type: "bytes";
        },
      ];
    },
    {
      name: "checkClaiming";
      docs: [
        "Debridge protocol allows to execute some Solana instructions from evm-like chains.",
        "Execution occurs using the debridge's `execute_external_call` instruction .",
        "The `execute_external_call` instruction invokes provided from evm instruction",
        "stored and verified in external_call_storage with Solana Cross-Program Invocations and",
        "[`anchor_lang::solana_program::program::invoke_signed`] function. Often there is a task to check",
        "that the program instruction is called from the `execute_external_call` instruction by",
        "[`anchor_lang::solana_program::program::invoke_signed`]. For this task you can use",
        "[`debridge_solana_sdk::check_claiming::check_execution_context`] function. For it you need to",
        "provide `submission` and `submission_authority` accounts and `source_chain_id`. Also you",
        "can check `native_sender`. It's user who call send function in source chain. With this",
        "function you can let two contracts communicate with each other.",
      ];
      accounts: [
        {
          name: "submission";
          isMut: false;
          isSigner: false;
        },
        {
          name: "submissionAuthority";
          isMut: false;
          isSigner: false;
        },
        {
          name: "instructions";
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
        {
          name: "sourceChainId";
          type: {
            array: ["u8", 32];
          };
        },
        {
          name: "nativeSender";
          type: {
            option: "bytes";
          };
        },
      ];
    },
  ];
  errors: [
    {
      code: 6000;
      name: "ChainNotSupported";
    },
    {
      code: 6001;
      name: "ChainSupportInfoDeserializingFailed";
    },
    {
      code: 6002;
      name: "MatchOverflowWhileCalculateInputAmount";
    },
    {
      code: 6003;
      name: "FailedToCalculateAmountWithFee";
    },
    {
      code: 6004;
      name: "NotEnoughAccountProvided";
    },
  ];
};

export const IDL: DebridgeInvokeExample = {
  version: "0.0.0",
  name: "debridge_invoke_example",
  instructions: [
    {
      name: "sendViaDebridge",
      docs: [
        "Debridge protocol allows transfer liquidity from Solana to other supported chains",
        "To send some token to other supported chain use [`debridge_solana_sdk::sending::invoke_debridge_send`]",
        "",
        "To check if the network is supported use [`debridge_solana_sdk::sending::is_chain_supported`]",
      ],
      accounts: [],
      args: [
        {
          name: "amount",
          type: "u64",
        },
        {
          name: "targetChainId",
          type: {
            array: ["u8", 32],
          },
        },
        {
          name: "receiver",
          type: "bytes",
        },
        {
          name: "isUseAssetFee",
          type: "bool",
        },
      ],
    },
    {
      name: "sendViaDebridgeWithNativeFixedFee",
      docs: [
        "Debridge protocol takes fix fee and transfer fee while sending liquidity.",
        "The fix fee by default is taken in native solana tokens.",
        "The default native fix fee amount is set in state account but it can set custom native",
        "fix amount for a specific chain in chain support info account.",
        "",
        "To get default native fix fee amount use [`debridge_solana_sdk::sending::get_default_native_fix_fee`]",
        "",
        "To get native fix fee amount for specific chain use [`debridge_solana_sdk::sending::get_chain_native_fix_fee`]",
        "",
        "To use native fix fee set [`debridge_solana_sdk::sending::SendIx`] `is_use_asset_fee` field to `false`",
      ],
      accounts: [],
      args: [
        {
          name: "amount",
          type: "u64",
        },
        {
          name: "targetChainId",
          type: {
            array: ["u8", 32],
          },
        },
        {
          name: "receiver",
          type: "bytes",
        },
      ],
    },
    {
      name: "sendViaDebridgeWithAssetFixedFee",
      docs: [
        "Debridge protocol takes fix fee and transfer fee while sending liquidity.",
        "The fix fee by default is taken in native solana tokens.",
        "But when transferring some tokens to certain networks, it is possible to pay in transferred tokens.",
        "It's called `asset_fix_fee`.",
        "",
        "To known `asset_fee` is available use [`debridge_solana_sdk::sending::is_asset_fee_available`]",
        "",
        "To get asset fix fee amount for specific chain use [`debridge_solana_sdk::sending::try_get_chain_asset_fix_fee`]",
        "",
        "To use asset fix fee set [`debridge_solana_sdk::sending::SendIx`] `is_use_asset_fee` field to `true`",
      ],
      accounts: [],
      args: [
        {
          name: "amount",
          type: "u64",
        },
        {
          name: "targetChainId",
          type: {
            array: ["u8", 32],
          },
        },
        {
          name: "receiver",
          type: "bytes",
        },
      ],
    },
    {
      name: "sendViaDebridgeWithExactAmount",
      docs: [
        "Debridge protocol takes fix fee and transfer fee while sending liquidity.",
        "If needed to get exact amount tokens in target chain, all fees will need to be added to sending amount.",
        "",
        "There are three types of fees in Debridge protocol: fixed fee, transfer fee, execution fee.",
        "",
        "Fixed fee is fixed amount for any send instruction. It's named asset fixed fee. The amount depends on target chain.",
        "To get asset fix fee amount for specific chain use [`debridge_solana_sdk::sending::try_get_chain_asset_fix_fee`]",
        "For some token fixed fee can be paid with sent tokens. In this case, you need to include this asset fixed fee in the final amount.",
        "",
        "Transfer fee is taken as part of sent tokens. To get the bps of transfer fee use [`debridge_solana_sdk::sending::get_transfer_fee`]",
        "To add transfer fee to current amount use [`debridge_solana_sdk::sending::add_transfer_fee`]",
        "",
        "Execution fee is reward for execute claim instruction in target chain. It can be zero if you want to run the instruction yourself.",
        "The recommended execution fee can be obtained using debridge sdk.",
        "",
        "To add to exact amount all fees charged during the send use [`debridge_solana_sdk::sending::add_all_fees`]",
      ],
      accounts: [],
      args: [
        {
          name: "exactAmount",
          type: "u64",
        },
        {
          name: "targetChainId",
          type: {
            array: ["u8", 32],
          },
        },
        {
          name: "receiver",
          type: "bytes",
        },
        {
          name: "executionFee",
          type: "u64",
        },
        {
          name: "isUseAssetFee",
          type: "bool",
        },
      ],
    },
    {
      name: "sendViaDebridgeWithExecutionFee",
      docs: [
        "Debridge protocol allows to anyone execute claim transaction in target chain. It allow to create",
        "fluent user experience when user only send tokens to other chain and automatically receives in another.",
        "",
        "User adds execution fee as reward for execution his claim transaction in target chain.",
        "The recommended execution fee can be obtained using debridge sdk.",
      ],
      accounts: [],
      args: [
        {
          name: "amount",
          type: "u64",
        },
        {
          name: "targetChainId",
          type: {
            array: ["u8", 32],
          },
        },
        {
          name: "receiver",
          type: "bytes",
        },
        {
          name: "executionFee",
          type: "u64",
        },
      ],
    },
    {
      name: "sendViaDebridgeWithExternalCall",
      docs: [
        "Debridge protocol allows not only to send tokens to another network,",
        "but also to use them to call any smart contract.",
        "",
        "Used `external_call` for this. For evm-like network it will be address of smart contract function and function's arguments",
        "packed in byte vector.",
        "",
        "To use external call function needed to initialize external call storage with",
        "[`debridge_solana_sdk::sending::invoke_init_external_call`] function and create `submission_params`",
        "with [`debridge_solana_sdk::sending::SendSubmissionParamsInput::with_external_call`] function.",
        "Besides external call needed to provide `fallback_address`. The `fallback_address' will be used",
        "if external call fails. On this address token received in target chain will transfer.",
        "",
        "A `execution_fee` is reward reward that will received for execution claim transaction in",
        "target chain. It can be set zero if external call will be claimed by yourself.",
      ],
      accounts: [],
      args: [
        {
          name: "amount",
          type: "u64",
        },
        {
          name: "targetChainId",
          type: {
            array: ["u8", 32],
          },
        },
        {
          name: "receiver",
          type: "bytes",
        },
        {
          name: "executionFee",
          type: "u64",
        },
        {
          name: "fallbackAddress",
          type: "bytes",
        },
        {
          name: "reservedFlag",
          type: {
            array: ["u8", 32],
          },
        },
        {
          name: "externalCall",
          type: "bytes",
        },
      ],
    },
    {
      name: "sendMessageViaDebridge",
      docs: [
        "deBridge protocol allows calling any smart contract in target chain without sending any tokens.",
        "You have to pay only a transfer fee for sending an execution fee to another chain.",
        "If you claim by yourself, set execution fee to zero, you don’t need to pay transfer fee at all.",
        "Only fixed fee will be taken.",
        "",
        "Used `external_call` for this. For evm-like network it will be address of smart contract",
        "function and function's arguments packed in byte vector.",
        "",
        "To send message with external call function use [`debridge_solana_sdk::sending::invoke_send_message`]",
        "function. This function will create external call storage, calculate transfer fee for",
        "transferring execution fee and send the message to target chain.",
        "Besides external call needed to provide `fallback_address`. The `fallback_address' will be used",
        "if external call fails. On this address token received in target chain will transfer.",
        "",
        "A `execution_fee` is reward reward that will received for execution claim transaction in",
        "target chain. It can be set zero if external call will be claimed by yourself.",
      ],
      accounts: [],
      args: [
        {
          name: "targetChainId",
          type: {
            array: ["u8", 32],
          },
        },
        {
          name: "receiver",
          type: "bytes",
        },
        {
          name: "executionFee",
          type: "u64",
        },
        {
          name: "fallbackAddress",
          type: "bytes",
        },
        {
          name: "message",
          type: "bytes",
        },
      ],
    },
    {
      name: "checkClaiming",
      docs: [
        "Debridge protocol allows to execute some Solana instructions from evm-like chains.",
        "Execution occurs using the debridge's `execute_external_call` instruction .",
        "The `execute_external_call` instruction invokes provided from evm instruction",
        "stored and verified in external_call_storage with Solana Cross-Program Invocations and",
        "[`anchor_lang::solana_program::program::invoke_signed`] function. Often there is a task to check",
        "that the program instruction is called from the `execute_external_call` instruction by",
        "[`anchor_lang::solana_program::program::invoke_signed`]. For this task you can use",
        "[`debridge_solana_sdk::check_claiming::check_execution_context`] function. For it you need to",
        "provide `submission` and `submission_authority` accounts and `source_chain_id`. Also you",
        "can check `native_sender`. It's user who call send function in source chain. With this",
        "function you can let two contracts communicate with each other.",
      ],
      accounts: [
        {
          name: "submission",
          isMut: false,
          isSigner: false,
        },
        {
          name: "submissionAuthority",
          isMut: false,
          isSigner: false,
        },
        {
          name: "instructions",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "sourceChainId",
          type: {
            array: ["u8", 32],
          },
        },
        {
          name: "nativeSender",
          type: {
            option: "bytes",
          },
        },
      ],
    },
  ],
  errors: [
    {
      code: 6000,
      name: "ChainNotSupported",
    },
    {
      code: 6001,
      name: "ChainSupportInfoDeserializingFailed",
    },
    {
      code: 6002,
      name: "MatchOverflowWhileCalculateInputAmount",
    },
    {
      code: 6003,
      name: "FailedToCalculateAmountWithFee",
    },
    {
      code: 6004,
      name: "NotEnoughAccountProvided",
    },
  ],
};
