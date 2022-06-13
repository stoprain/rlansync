#[cfg(test)]
mod tests {

    use crate::protos;
    use protobuf::Message;
    use protos::generated_with_pure::example::{GetRequest, FileInfoRequest, FileDataRequest};
    use protobuf::well_known_types::any::Any;
    use protobuf::MessageField;

    #[test]
    fn test_protobuf() {
        let mut out_msg = FileInfoRequest::new();
        out_msg.from = 12345;
    
        let mut outm = GetRequest::new();
        outm.details = MessageField::some(Any::pack(&out_msg).unwrap());
    
        let out_bytes: Vec<u8> = out_msg.write_to_bytes().unwrap();
    
        let in_msg = GetRequest::parse_from_bytes(&out_bytes).unwrap();

        assert!(outm.details.is::<FileInfoRequest>());
        if outm.details.is::<FileInfoRequest>() {
            let request = outm.details.unpack::<FileInfoRequest>().unwrap().unwrap();
            assert_eq!(request.from, 12345)
        }
    }
}