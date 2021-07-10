mod bprotocol;

#[cfg(test)]
mod tests {
    use super::bprotocol;
    use bencode::ByteBuf;

    // Dumped with wireshark.
    const FIND_NODE_REQUEST: &[u8] = b"64313a6164323a696432303abd7b477cfbcd10f30b705da20201e7101d8df155363a74617267657432303abd7b477cfbcd10f30b705da20201e7101d8df15565313a71393a66696e645f6e6f6465313a74323a0005313a79313a7165";
    const GET_PEERS_REQUEST: &[u8] = b"64313a6164323a696432303abd7b477cfbcd10f30b705da20201e7101d8df155393a696e666f5f6861736832303acab507494d02ebb1178b38f2e9d7be299c86b86265313a71393a6765745f7065657273313a74323a0006313a79313a7165";
    const FIND_NODE_RESPONSE: &[u8] = b"64313a7264323a696432303a3c00727348b3b8ed70baa1e1411b3869d8481321353a6e6f6465733230383a67a312defb7d429086bfdcd5a209684ee13f59615cbe360bc8d567a312defb7d429086bfdcd5a209684ee13f59615cbe360bc8d567a312defb7d429086bfdcd5a209684ee13f59615cbe360bc8d567a312defb7d429086bfdcd5a209684ee13f59615cbe360bc8d567a312defb7d429086bfdcd5a209684ee13f59615cbe360bc8d567a312defb7d429086bfdcd5a209684ee13f59615cbe360bc8d567a312defb7d429086bfdcd5a209684ee13f59615cbe360bc8d567a312defb7d429086bfdcd5a209684ee13f59615cbe360bc8d565313a74323a0005313a76343a4a420000313a79313a7265";
    const FIND_NODE_RESPONSE_2: &[u8] = b"64323a6970363a081ab440e935313a7264323a696432303a32f54e697351ff4aec29cdbaabf2fbe3467cc267353a6e6f6465733431363a54133f7f6d77567ff210fe88d49839107d1a955956aaa625e9ee438e4a0af6b324d9672886052c856b26b25835a689afbbdf5436b643eb20605e1d18f848b32cd275a117afb52d3a474d18541ae18dd20d3fbd936983af4ea87135d785d0661de2f4c4bf7925c59269105c05caa68658851c018d8890f73604e334afdfb8e556fd7ca8f3e0211bd2af91c4af4eee69415a273c0bd1c2b02e8b9ba827139b6c6ebc6dcb6ee53aac3c5147530a432e1b62c9116e1316e9364d7fd2f10f2499f47e862d847937e39a51aed74bb6e8f1c491d520868f1893aaa007d1af19b5328f1b4840759e5743aa59a6bf090c76b846145c6895303b7a49be387fd609a9212eb6541b1ae1fd2ddcf776b4688dd359c8157120809ac8b6651e5e6e8d58b4a80fa124e1f4ed536d61e4ee25d5a702fc8ab70cdf45852708c999215cc406c4caa862bcd0a6b88e58128d2b280ac74631b3591ae1fa4484a5560c31de4fc046b97b4c6ac31dc324ab2ef20952049bfcecdbc8cf79e4cfd378a89779c605559b79b8ae25ba326249e5629f7b9cc0ad33143832e1bca63da63cdb8a940117f0adc2c41965313a74323a0002313a79313a7265";
    const FIND_NODE_RESPONSE_3: &[u8] = b"64323a6970363a081ab440e935313a7264323a696432303a32f54e697351ff4aec29cdbaabf2fbe3467cc267353a6e6f6465733431363a26d4302a32aecf28f3fee9f6caf8867d762e28b963b5a531c4917373b33fb43c9d7c0d3daf45ee22ab947d4511c054364d4a904464878fc4a31e88b41d7ea953f7dc91d8017dafee5d0f8a4d2fa19fd3ec1c37c6807cad0a5601698909e7a487532fb9408928afaa7ca5e376bee87c4caafa88f2f9a9cc2ed992cd48be68771b48bb6efc225561c00dc3f40d04ab08d93c21a1b89097bd06fa4d1d122d6f1d86e041a5525a69b26d265d039cd52c8bebc923bf1bc3e9f71c7ed05e349d54465cca22233147f21d4c1cc531e461254249ea653909abe367bc25efab70bbe28cd38cbafc2e6db11df5d66bc20bc8a4c9490d84bf29f09ceb44c230dd2ced8b5cec47c71ae1ff66e9ed230e165873b0bef32163ad52c66edce28a7c9c8ae8647af27ba1eac73737ac167e21ed9116b1ef8104a7c28f89606be6f36d7584b791128793e8f8a0e6b48897a6463532547e400ef3a7067237d4d77bf40f1c09773ea85dd269adf35eeebca89b6993cdb116c0512abc2cbc74973d5e5f09940d0bbdf4e047ce15101ae13d794b1230188404a9fd2a5a10ccefb0622057bc6d7eeae5fb8565313a74323a0003313a79313a7265";

    const WHAT_IS_THAT: &[u8]= b"64313a6164323a696432303abd7b477cfbcd10f30b705da20201e7101d8df155393a696e666f5f6861736832303acab507494d02ebb1178b38f2e9d7be299c86b86265313a71393a6765745f7065657273313a74323a0007313a79313a7165";

    fn debug_hex_bencode(name: &str, data: &[u8]) {
        println!("{}", name);
        let data = hex::decode(data).unwrap();

        println!(
            "{:#?}",
            bencode::dyn_from_bytes::<ByteBuf>(data.as_slice()).unwrap()
        );
    }

    #[test]
    fn deserialize_request_find_node() {
        let req = hex::decode(FIND_NODE_REQUEST).unwrap();
        dbg!(bprotocol::deserialize_message::<ByteBuf>(&req).unwrap());
    }

    #[test]
    fn deserialize_request_get_peers() {
        let req = hex::decode(GET_PEERS_REQUEST).unwrap();
        dbg!(bprotocol::deserialize_message::<ByteBuf>(&req).unwrap());
    }

    #[test]
    fn deserialize_response_find_node() {
        let req = hex::decode(FIND_NODE_RESPONSE).unwrap();
        dbg!(bprotocol::deserialize_message::<ByteBuf>(&req).unwrap());
    }

    #[test]
    fn deserialize_response_find_node_2() {
        let req = hex::decode(FIND_NODE_RESPONSE_2).unwrap();
        dbg!(bprotocol::deserialize_message::<ByteBuf>(&req).unwrap());
    }

    #[test]
    fn deserialize_response_find_node_3() {
        let req = hex::decode(FIND_NODE_RESPONSE_3).unwrap();
        dbg!(bprotocol::deserialize_message::<ByteBuf>(&req).unwrap());
    }

    #[test]
    fn deserialize_request_what_is_that() {
        let req = hex::decode(WHAT_IS_THAT).unwrap();
        dbg!(bprotocol::deserialize_message::<ByteBuf>(&req).unwrap());
    }

    #[test]
    fn deserialize_bencode_packets_captured_from_wireshark() {
        debug_hex_bencode("req: find_node", FIND_NODE_REQUEST);
        debug_hex_bencode("req: get_peers", GET_PEERS_REQUEST);
        debug_hex_bencode("resp from the requesting node", FIND_NODE_RESPONSE);
        debug_hex_bencode("resp from some random IP", FIND_NODE_RESPONSE_2);
        debug_hex_bencode("another resp from some random IP", FIND_NODE_RESPONSE_3);
        debug_hex_bencode("req to another node", WHAT_IS_THAT);
    }
}
