use std::fmt;
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use crate::OpenoceanError;

// https://apis.openocean.finance/developer/apis/supported-chains


#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Chain {
    Eth,
    Bsc,
    ZkSyncEra,
    Polygon,
    Base,
    Linea,
    Fantom,
    Avalanche,
    Arbitrum,
    Optimism,
    Moonriver,
    Aurora,
    Cronos,
    Harmony,
    Kava,
    MetisAndromeda,
    Celo,
    Telos,
    PolygonZkEVM,
    Gnosis,
    OpBNB,
    Mantle,
    Manta,
    Scroll,
    Blast,
    Mode,
    Rootstock,
    Sei,
    Gravity,
    Apechain,
    Sonic,
    Berachain,
    MonadTestnet,
    UniChain,
    Flare,
    Swell,
    HyperEVM,
    Plume,
    TAC,

    // no evm chains
    Solana,
    Ontology,
    Near,
    Starknet,
    // Sui,
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Chain::Eth => "eth",
            Chain::Bsc => "bsc",
            Chain::ZkSyncEra => "zksync",
            Chain::Polygon => "polygon",
            Chain::Base => "base",
            Chain::Linea => "linea",
            Chain::Fantom => "fantom",
            Chain::Avalanche => "avax",
            Chain::Arbitrum => "arbitrum",
            Chain::Optimism => "optimism",
            Chain::Moonriver => "moonriver",
            Chain::Aurora => "aurora",
            Chain::Cronos => "cronos",
            Chain::Harmony => "harmony",
            Chain::Kava => "kava",
            Chain::MetisAndromeda => "metis",
            Chain::Celo => "celo",
            Chain::Telos => "telos",
            Chain::PolygonZkEVM => "polygon_zkevm",
            Chain::Gnosis => "gnosis",
            Chain::OpBNB => "opbnb",
            Chain::Mantle => "mantle",
            Chain::Manta => "manta",
            Chain::Scroll => "scroll",
            Chain::Blast => "blast",
            Chain::Mode => "mode",
            Chain::Rootstock => "rootstock",
            Chain::Sei => "sei",
            Chain::Gravity => "gravity",
            Chain::Apechain => "ape",
            Chain::Sonic => "sonic",
            Chain::Berachain => "bera",
            Chain::MonadTestnet => "monad",
            Chain::UniChain => "uni",
            Chain::Flare => "flare",
            Chain::Swell => "swell",
            Chain::HyperEVM => "hyperevm",
            Chain::Plume => "plume",
            Chain::TAC => "tac",
            // no evm chains
            Chain::Solana => "solana",
            Chain::Ontology => "ont",
            Chain::Near => "near",
            Chain::Starknet => "starknet",
            // Sui => "sui",
        };
        f.write_str(s)
    }
}


impl TryFrom<String> for Chain {
    type Error = OpenoceanError;

    fn try_from(chain: String) -> Result<Self, Self::Error> {
        match chain.as_str() {
            "1" | "eth" => Ok(Chain::Eth),
            "56" | "bsc" => Ok(Chain::Bsc),
            "137" | "polygon" => Ok(Chain::Polygon),
            "42161" | "arbitrum" => Ok(Chain::Arbitrum),
            "10" | "optimism" => Ok(Chain::Optimism),
            "250" | "fantom" => Ok(Chain::Fantom),
            "43114" | "avalanche" => Ok(Chain::Avalanche),
            "100" | "gnosis" => Ok(Chain::Gnosis),
            "59144" | "cronos" => Ok(Chain::Cronos),
            "1666600000" | "harmony" => Ok(Chain::Harmony),
            "2000" | "kava" => Ok(Chain::Kava),
            "1088" | "metis" => Ok(Chain::MetisAndromeda),
            "42220" | "celo" => Ok(Chain::Celo),
            "42261" | "telos" => Ok(Chain::Telos),
            "1313161554" | "polygon_zkevm" => Ok(Chain::PolygonZkEVM),
            "500" | "opbnb" => Ok(Chain::OpBNB),
            "501" | "mantle" => Ok(Chain::Mantle),
            "502" | "manta" => Ok(Chain::Manta),
            "503" | "scroll" => Ok(Chain::Scroll),
            "504" | "blast" => Ok(Chain::Blast),
            "505" | "mode" => Ok(Chain::Mode),
            "506" | "rootstock" => Ok(Chain::Rootstock),
            "507" | "sei" => Ok(Chain::Sei),
            "508" | "gravity" => Ok(Chain::Gravity),
            "509" | "ape" => Ok(Chain::Apechain),
            "510" | "sonic" => Ok(Chain::Sonic),
            "511" | "bera" => Ok(Chain::Berachain),
            _ => Err(OpenoceanError::Internal(format!("Unsupported chain: {}", chain))),
        }
    }
}