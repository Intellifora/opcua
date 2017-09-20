// This file was autogenerated from Opc.Ua.NodeSet2.Part14.xml
// DO NOT EDIT THIS FILE

#[allow(unused_imports)]
use std::str::FromStr;

#[allow(unused_imports)]
use opcua_types::*;
#[allow(unused_imports)]
use address_space::types::*;

#[allow(unused_variables)]
pub fn populate_address_space(address_space: &mut AddressSpace) {
    {
        // Object
        let browse_name = "Default XML";
        let display_name = "Default XML";
        let description = "";
        let node_id = NodeId::new(0, 14801);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node);
        address_space.insert_reference(&NodeId::new(0, 14532), &node_id, ReferenceTypeId::HasEncoding);
        address_space.insert_reference(&node_id, &NodeId::new(0, 14826), ReferenceTypeId::HasDescription);
        address_space.insert_reference(&node_id, &NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        let browse_name = "Default Binary";
        let display_name = "Default Binary";
        let description = "";
        let node_id = NodeId::new(0, 14845);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node);
        address_space.insert_reference(&NodeId::new(0, 14532), &node_id, ReferenceTypeId::HasEncoding);
        address_space.insert_reference(&node_id, &NodeId::new(0, 14870), ReferenceTypeId::HasDescription);
        address_space.insert_reference(&node_id, &NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // DataType
        let browse_name = "EnumField";
        let display_name = "EnumField";
        let description = "";
        let node_id = NodeId::new(0, 14532);
        let node = DataType::new(&node_id, browse_name, display_name, description, false);
        address_space.insert(node);
        address_space.insert_reference(&NodeId::new(0, 7594), &node_id, ReferenceTypeId::HasSubtype);
    }
}
