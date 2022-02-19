use juniper_codegen::GraphQLObject;
use serde::Deserialize;
use tinkc::grpc::hardware;

#[derive(Debug, Deserialize, GraphQLObject)]
pub struct Dhcp {
    hostname: String,
}

impl TryFrom<hardware::hardware::Dhcp> for Dhcp {
    type Error = eyre::Error;
    fn try_from(dhcp: hardware::hardware::Dhcp) -> std::result::Result<Dhcp, Self::Error> {
        let hardware::hardware::Dhcp { hostname, .. } = dhcp;
        Ok(Dhcp { hostname })
    }
}

#[derive(Debug, Deserialize, GraphQLObject)]
pub struct Hardware {
    id: String,
    network: Option<Network>,
}

impl TryFrom<tinkc::Hardware> for Hardware {
    type Error = eyre::Error;
    fn try_from(hw: tinkc::Hardware) -> std::result::Result<Hardware, Self::Error> {
        let tinkc::Hardware { id, network, .. } = hw;
        let network = match network {
            Some(n) => n.try_into().ok(),
            _ => None,
        };
        Ok(Hardware { id, network })
    }
}

#[derive(Debug, Deserialize, GraphQLObject)]
pub struct Interface {
    dhcp: Option<Dhcp>,
}

impl TryFrom<hardware::hardware::network::Interface> for Interface {
    type Error = eyre::Error;
    fn try_from(
        interface: hardware::hardware::network::Interface,
    ) -> std::result::Result<Interface, Self::Error> {
        let hardware::hardware::network::Interface { dhcp, .. } = interface;
        let dhcp = match dhcp {
            Some(dhcp) => Some(dhcp.try_into()?),
            _ => None,
        };
        Ok(Interface { dhcp })
    }
}

#[derive(Debug, Deserialize, GraphQLObject)]
pub struct Network {
    interfaces: Vec<Interface>,
}

impl TryFrom<hardware::hardware::Network> for Network {
    type Error = eyre::Error;
    fn try_from(network: hardware::hardware::Network) -> std::result::Result<Network, Self::Error> {
        let hardware::hardware::Network { interfaces, .. } = network;
        let interfaces = interfaces
            .into_iter()
            .filter_map(|interface| interface.try_into().ok())
            .collect();
        Ok(Network { interfaces })
    }
}
