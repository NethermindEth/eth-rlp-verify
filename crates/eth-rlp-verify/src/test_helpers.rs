use eth_rlp_types::BlockHeader;

pub fn create_test_block_header_pectra() -> BlockHeader {
    BlockHeader {
        block_hash: "0xbfa14ad39de89b0de89a0d9e78efebae792eae93ee46414ed98ee790ce8ed8b3"
            .to_string(),
        number: 7839744,
        gas_limit: 36000000,
        gas_used: 17786126,
        nonce: "0x0000000000000000".to_string(),
        transaction_root: Some(
            "0x0278307f8545a019516ec439e73404f952436abbdf21d69720a613fef8938d5f".to_string(),
        ),
        receipts_root: Some(
            "0xc7bbdde3e099ede140e4364e3205a16dc68bdd5531af1596789537bbd2ff1041".to_string(),
        ),
        state_root: Some(
            "0x724617b7999d13469c068ced397d3d37a1432a8db7f156ed81d8f78c0a61a7f2".to_string(),
        ),
        base_fee_per_gas: Some("0x120c4011".to_string()),
        parent_hash: Some(
            "0x53df8defa12fa604f56a0b490e45f8eaebb12c3a9aed12bc52d405600e606338".to_string(),
        ),
        miner: Some("0x3826539cbd8d68dcf119e80b994557b4278cec9f".to_string()),
        logs_bloom: Some("0x0291928e4009a0159203220927940094062800a00c093a669298e00dc5bf42022a50500200334a06000029401634803691d407868254f8847c00d08d4c6604842c62588e4b106309407022cad848de348041474480540286130a080291b202110f100c409204120804a022800220084183500d004168061146246012215836041aee02124e300d3060ed400b99052084a0c0a18b886802082ca0509c4482001c028a788efd0480a9d27a105940000c5900cc8601a2031380855453c46f79128388548f0224603490610a80310087430a08946c84e00d00304409910211a071210550352d0c082800c5361f8628b9a6c3c2b275304c70834200da43181a486021".to_string()),
        difficulty: Some("0x0".to_string()),
        totaldifficulty: Some("0xd8fb2...".to_string()),
        sha3_uncles: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        timestamp: Some("0x67c8a89c".to_string()),
        extra_data: Some("0x".to_string()),
        mix_hash: Some(
            "0x31d5334ad1f9e04153891a4697863a1c9e80f37c66d1d421923c07184c45b4d5".to_string(),
        ),
        ommers_hash: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        withdrawals_root: Some(
            "0x707609a48eb417ad00247824863dcae7437a4ae780719e77a1d6788f05c1a3b8".to_string(),
        ),
        blob_gas_used: Some("0xe0000".to_string()),
        excess_blob_gas: Some("0x60000".to_string()),
        parent_beacon_block_root: Some(
            "0x4f5861bbeae03efc716a3b9cfaf4473aca4048e489a9b77102edb2117c4b8217".to_string(),
        ),
        request_hash: Some("0xe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string()),
    }
}

pub fn create_test_block_header_shapella() -> BlockHeader {
    BlockHeader {
        block_hash: "0xb49fa930183f0fa8ca46244e946a635637b90c374d845e9c68dd7fd66cb6e0a1"
            .to_string(),
        number: 17034871,
        gas_limit: 30000000,
        gas_used: 29970493,
        nonce: "0x0000000000000000".to_string(),
        transaction_root: Some(
            "0xd3deb43136a95fac91dc5a451caf62b18f0d3ffa5d8744b769ac5df6d8ef40bc".to_string(),
        ),
        receipts_root: Some(
            "0xe4001611ca993adcb8948b4f114bc0985948bee0fef32c4e2292ad87356afd58".to_string(),
        ),
        state_root: Some(
            "0x1d49150d23cc59f06491e138b7a9bf0d307bcd7bb2bab6e38754740931d6c3ef".to_string(),
        ),
        base_fee_per_gas: Some("0x4b5b025b0".to_string()),
        parent_hash: Some(
            "0xe22c56f211f03baadcc91e4eb9a24344e6848c5df4473988f893b58223f5216c".to_string(),
        ),
        miner: Some("0x0b70b578abd96aab5e80d24d1f3c28dbde14356a".to_string()),
        logs_bloom: Some("0x9479b5dce9a45ebc5af9eee4f1a9cb73dfb3437b2b7edff2a5f36addcfa4319e74212ff4d469c756dfddbeb6db79d5ab5effee57ac57fdfdbbe30a1fd8ffa9fdb9ddfb19771ffdbd7eff4bdbd3d5f97fdf571edeeafefe5a6eb55eceffd739b27ffcff7f16a38dbff45ddd25fb9d7b9bef9bae37e0bb6f7dfbf5fbf3cdfd5dbdbe57ffdfe7ef77ddb7fef267af53587bfb7ffcb3f9f673fc7eababe5fbfe77eddff945e347a6fffdfbfbe8fd7ffee79efcfcce5d77a66db3f9fabe9f3b8b1d79f9a99bb7af11f93f6f9b736f2cbf5697d57f4b6de478bdfef6759dfbbe5bebee7cfff7e9df8acdd46f9777fb47edfaf0b5f5ba74393acff5d24bfcf389fb9eff".to_string()),
        difficulty: Some("0x0".to_string()),
        totaldifficulty: Some("0xd8fb2...".to_string()),
        sha3_uncles: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        timestamp: Some("0x64373087".to_string()),
        extra_data: Some("0x4e65746865726d696e64".to_string()),
        mix_hash: Some(
            "0xd7a4a06e28abcc8ac4e0bab5f0a7e60ea7a0c3de93b2f7e7a4cc3c9a79e60186".to_string(),
        ),
        ommers_hash: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        withdrawals_root: Some(
            "0xc32381c919dad80afe8fe0df79460418e350725a63f67c55b27ee168ef464e5d".to_string(),
        ),
        blob_gas_used: Some("0xa0000".to_string()),
        excess_blob_gas: Some("0x20000".to_string()),
        parent_beacon_block_root: Some(
            "0x2b5b8c2d329148bc5d724cac0a7abc557f93471c24ea027b119ecedb937c0045".to_string(),
        ),
        request_hash: None,
    }
}

pub fn create_test_block_header_paris() -> BlockHeader {
    BlockHeader {
        block_hash: "0xa49fa930183f0fa8ca46244e946a635637b90c374d845e9c68dd7fd66cb6e051"
            .to_string(),
        number: 15537395,
        gas_limit: 30000000,
        gas_used: 29982083,
        nonce: "0x0000000000000000".to_string(),
        transaction_root: Some(
            "0x5c56184fbce74e9c98d2a51aa2110963396047d84e8ce1ae1785337255cd11e0".to_string(),
        ),
        receipts_root: Some(
            "0x1707e457973ce280debe93f5d478663d97ad192beea1f2fc65f391db80226e5e".to_string(),
        ),
        state_root: Some(
            "0x2ca38a39c5517f658d107c19550334a9820a7393d14857f2c0e0458292668d64".to_string(),
        ),
        base_fee_per_gas: Some("0xcc8ac8283".to_string()),
        parent_hash: Some(
            "0x56a9bb0302da44b8c0b3df540781424684c3af04d0b7a38d72842b762076a664".to_string(),
        ),
        miner: Some("0x0b3b161b8abeb6b04cb95c3e6047f80c120a0292".to_string()),
        logs_bloom: Some(
            "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                .to_string(),
        ),
        difficulty: Some("0x0".to_string()),
        totaldifficulty: Some("0xc8fb2...".to_string()),
        sha3_uncles: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        timestamp: Some("0x6322c97f".to_string()),
        extra_data: Some("0x".to_string()),
        mix_hash: Some(
            "0xfd7be062e87b2193dff12ca89e90bbf61684e6676df4e86cca6730237863dd23".to_string(),
        ),
        ommers_hash: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        withdrawals_root: None,
        blob_gas_used: None,
        excess_blob_gas: None,
        parent_beacon_block_root: None,
        request_hash: None,
    }
}

pub fn create_test_block_header_london() -> BlockHeader {
    BlockHeader {
        block_hash: "0x9b83c12c69edb74f6c8dd5d052765c1adf940e320bd1291696e6fa07829eee71".to_string(),
        number: 12965001,
        gas_limit: 29999798,
        gas_used: 29985144,
        nonce: "0x0956e895d988798e".to_string(),
        transaction_root: Some("0x03c97f958cc4db3cc60def5ce1e83aaf1490837f5f57c529a6ccffef0d201edb".to_string()),
        receipts_root: Some("0x2335850563dbf51f65a37508f2fdd9da1780f70cfa46734107a2e86a9fde46d7".to_string()),
        state_root: Some("0x0180d59eb0855ef6dbca806fbe81491bea252ab2e0d3a8bb8786326d598e3cd9".to_string()),
        base_fee_per_gas: Some("0x430da58e".to_string()),
        parent_hash: Some("0x9b83c12c69edb74f6c8dd5d052765c1adf940e320bd1291696e6fa07829eee71".to_string()),
        ommers_hash: Some("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string()),
        miner: Some("0x829bd824b016326a401d083b33d092293333a830".to_string()),
        logs_bloom: Some("0x74adf8cfdd0a1ddf12f3d6d5bbd79cab73a19b6986fc007932d9acffafebb747debf512456c87e9afffa5f40fd21ad403b97f3b38e86e9e9db62433eb2b6f8547ad677fdab07f1adcb83686fb37db9ea7acb113f0d74b397324d9cfbf8f33cb3dbfb0d256bcbdaf608dd7b1ac168ee40e322b69bf675a6f4fbbbbe72dccbdd88fab28e7d94685c34bffc9bd1ff98ef777af7ff9793de951d336a1b75acbc7f11ce9dac7e9942ab6a363b4fbebbc3d738dbee5a993fa7c87adce26cbeddfdfcf4d59bba977fb7514a3da550c0b21f399e8bf56778c7dfdcfeeb2457abef1fe63eaf38ecbabdae6c237afd34378163feb6ccdb42f56782cd474bdf9ee9fadb94b4".to_string()),
        difficulty: Some("0x1b81c23e05b218".to_string()),
        totaldifficulty: Some("0x608af5dd578251af429".to_string()),
        sha3_uncles: Some("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string()),
        timestamp: Some("0x610bdab3".to_string()),
        extra_data: Some("0xe4b883e5bda9e7a59ee4bb99e9b1bc030521".to_string()),
        mix_hash: Some("0xcb3166ebb1888430069b769145b20ba5e3a55f32fd2fa39f0ebdc08d60b4557e".to_string()),
        withdrawals_root: None,
        blob_gas_used: None,
        excess_blob_gas: None,
        parent_beacon_block_root: None,
        request_hash: None,
    }
}

pub fn create_test_block_header_cancun_sepolia() -> BlockHeader {
    BlockHeader {
        block_hash: "0x3919e733febe493f7c84bcc03629dada9c1995ab45a92503747a52feef24ac31".to_string(),
        number: 5187062,
        gas_limit: 30000000,
        gas_used: 11462046,
        nonce: "0x0000000000000000".to_string(),
        transaction_root: Some(
            "0x0088f926e6f6c4473e0e459037644419dd02838d56169c6096f4cd29edb2685a".to_string(),
        ),
        receipts_root: Some(
            "0xe30ec9875fbd9b55a34e5e3928d132a50b399f2259ea43f58976872d99256fff".to_string(),
        ),
        state_root: Some(
            "0x9e404c0b23a883cd8f4e3885f008d6cf541daae9d49c3aaeee908d680d3dbc3f".to_string(),
        ),
        base_fee_per_gas: Some("0x100a133fc".to_string()),
        parent_hash: Some(
            "0xcf55efa9b2fb5c657a713e8bb7c68e30a94de080620e693abc10670ae03a6553".to_string(),
        ),
        miner: Some("0x9b7e335088762ad8061c04d08c37902abc8acb87".to_string()),
        logs_bloom: Some("0x8020960e9c3141c0615930221311011084234026d470a0424a91a01804e00110600681108802149b82208800090488a28a80018604a69312538812122170810212080002908792206020300a0800248184001a058080562438020141d44470004a4828092649465480c040e114002906040905c088000460300080171208c000021204088800650d26011100190152c8010144057882d5141113c881844018410e08120040a48c0e404123000014804e17e000100806c51434e72c321690200145a210030e0a8048445804880c1790201214135418400005a20484802000240d84d2872ce249209081080001e8940004040250906a6d81438076220400201189".to_string()),
        difficulty: Some("0x0".to_string()),
        totaldifficulty: Some("0x3c656d23029ab0".to_string()),
        sha3_uncles: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        timestamp: Some("0x65b7d5c8".to_string()),
        extra_data: Some("0xd883010d0b846765746888676f312e32312e36856c696e7578".to_string()),
        mix_hash: Some(
            "0x6fc3d4b7ef0bbb1543928db7aa9aca670b77801973a86d3c08e2aae4cc156e84".to_string(),
        ),
        ommers_hash: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        withdrawals_root: Some(
            "0x7fc341eee4ab6fdd82595aa3c6b8e720a8b75396b26343e8287f8c35bd9c5bc6".to_string(),
        ),
        blob_gas_used: Some("0xc0000".to_string()),
        excess_blob_gas: Some("0x2a0000".to_string()),
        parent_beacon_block_root: Some(
            "0x6e975ee5654a4312c7ea1428cae73b45bff6c26f7899d6bbadcd3d6c3b583bbd".to_string(),
        ),
        request_hash: None,
    }
}

pub fn create_test_block_header_cancun() -> BlockHeader {
    BlockHeader {
        block_hash: "0xc30bad27d3bcaece0a3676bbf0cfd3f2711d8e9bf82b03d6c8eaf7d38cc26218".to_string(),
        number: 21360407,
        gas_limit: 30000000,
        gas_used: 23136748,
        nonce: "0x0000000000000000".to_string(),
        transaction_root: Some(
            "0x7e8cb150b22634aa93698d7474295d1e5cd20058f5dd87937d614e343c522ce9".to_string(),
        ),
        receipts_root: Some(
            "0x920933bdc0a76aa6e20f9055c5121fec90b3026407850fe0e3be9e501aa4aaa3".to_string(),
        ),
        state_root: Some(
            "0xd9f5fd68ad4eb866f72c2c0cc92522fcb2f8217de90f571b12af014595a712d1".to_string(),
        ),
        base_fee_per_gas: Some("0x221181aba".to_string()),
        parent_hash: Some(
            "0xd961036e50def39e5596c000f882ff8c01480238d7b5cef205dcb9eb2564f1e9".to_string(),
        ),
        miner: Some("0x4838b106fce9647bdf1e7877bf73ce8b0bad5f97".to_string()),
        logs_bloom: Some("0x56bfda6a6d3bbafbf2e9f1fcfdf412f5fb4aba67b8537dec59ff736ae7f28912e457cfe793f818b24e71bff8df3e9dfc6e63be6dcf786ddedee3f7deecfb3b65fa35d5bc4bbdeba87cf77b7fd2716affcc2b87bfdf6fddf26d7e1f79ffebee16da077f073f7e89ef85fc9ffe7848b9d7f2bffcec8f153efaba71ddfecaebdebdc9abaf7f1fdfd5fbb0fbab771b7a7f3ea6d17f7fefe60eeff8aadc67e7d8ffababffb56ffabd7cde6bda7bf6c1fc5de57657cfa3b9f94eaf437c4faa3688e26f711c6a3ef55d7b626034f9cf75f8fbe76be8f776bd7679b64fb7ed6ee9ebe4f6503bee7b1d98d60e674476b228fd1b7ed5eeffcfd3b9bfc45c6bb168d6fe7e87".to_string()),
        difficulty: Some("0x0".to_string()),
        totaldifficulty: Some("0xc70d815d562d3cfa955".to_string()),
        sha3_uncles: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        timestamp: Some("0x675e4067".to_string()),
        extra_data: Some("0x546974616e2028746974616e6275696c6465722e78797a29".to_string()),
        mix_hash: Some(
            "0xba0c445804efa055afaf3e3c70982df08f17b0209cb7cbfb5b9ac769ea14bc36".to_string(),
        ),
        ommers_hash: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        withdrawals_root: Some(
            "0x4a45d936f39861fe6ce5285973f70b5d836989cc178e86f6f613b1741e790cec".to_string(),
        ),
        blob_gas_used: Some("0x80000".to_string()),
        excess_blob_gas: Some("0x3740000".to_string()),
        parent_beacon_block_root: Some(
            "0x49c85e7f2d121cc6f8f55e1265785690e63e0769285c2cfc47112a1406631e47".to_string(),
        ),
        request_hash: None,
    }
}
