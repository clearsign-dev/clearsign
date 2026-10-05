"""The chains the benchmark samples, and where it reads them from.

`rpc` serves blocks and transactions. `logs` is an endpoint that answers
eth_getLogs filtered by topic alone (many public endpoints refuse that and
demand a contract address), with the largest block range it accepted when
probed on 4-5 Oct 2026. Public endpoints change; when one stops answering,
replace it here and say so in the commit, because each corpus manifest records
the endpoints it was collected from.
"""

CHAINS = [
    # The twelve networks the application offers, first.
    dict(id=1, path="eth", name="Ethereum",
         rpc="https://ethereum-rpc.publicnode.com", logs="https://rpc.mevblocker.io", window=1000),
    dict(id=42161, path="arb1", name="Arbitrum One",
         rpc="https://arbitrum-one-rpc.publicnode.com", logs="https://arb1.arbitrum.io/rpc", window=10000),
    dict(id=8453, path="base", name="Base",
         rpc="https://base-rpc.publicnode.com", logs="https://mainnet.base.org", window=1000),
    dict(id=10, path="oeth", name="OP Mainnet",
         rpc="https://optimism-rpc.publicnode.com", logs="https://mainnet.optimism.io", window=5000),
    dict(id=137, path="pol", name="Polygon",
         rpc="https://polygon.drpc.org", logs="https://polygon.drpc.org", window=100),
    dict(id=56, path="bnb", name="BNB Chain",
         rpc="https://bsc-rpc.publicnode.com", logs="https://bsc.blockrazor.xyz", window=25),
    dict(id=43114, path="avax", name="Avalanche C-Chain",
         rpc="https://avalanche-c-chain-rpc.publicnode.com", logs="https://api.avax.network/ext/bc/C/rpc", window=2000),
    dict(id=100, path="gno", name="Gnosis",
         rpc="https://gnosis-rpc.publicnode.com", logs="https://rpc.gnosischain.com", window=1000),
    dict(id=42220, path="celo", name="Celo",
         rpc="https://forno.celo.org", logs="https://forno.celo.org", window=5000),
    dict(id=534352, path="scr", name="Scroll",
         rpc="https://rpc.scroll.io", logs="https://rpc.scroll.io", window=5000),
    dict(id=59144, path="linea", name="Linea",
         rpc="https://rpc.linea.build", logs="https://rpc.linea.build", window=5000),
    dict(id=11155111, path="sep", name="Sepolia (test network)",
         rpc="https://ethereum-sepolia-rpc.publicnode.com", logs="https://1rpc.io/sepolia", window=50),
    # Further chains Safe's service indexes and a public endpoint answers for.
    dict(id=324, path="zksync", name="zkSync Era",
         rpc="https://mainnet.era.zksync.io", logs="https://mainnet.era.zksync.io", window=5000),
    dict(id=5000, path="mantle", name="Mantle",
         rpc="https://rpc.mantle.xyz", logs="https://rpc.mantle.xyz", window=5000),
    dict(id=146, path="sonic", name="Sonic",
         rpc="https://rpc.soniclabs.com", logs="https://rpc.soniclabs.com", window=5000),
    dict(id=130, path="unichain", name="Unichain",
         rpc="https://mainnet.unichain.org", logs="https://mainnet.unichain.org", window=5000),
    dict(id=80094, path="berachain", name="Berachain",
         rpc="https://rpc.berachain.com", logs="https://rpc.berachain.com", window=5000),
    dict(id=57073, path="ink", name="Ink",
         rpc="https://rpc-gel.inkonchain.com", logs="https://rpc-gel.inkonchain.com", window=5000),
    dict(id=480, path="wc", name="World Chain",
         rpc="https://worldchain-mainnet.g.alchemy.com/public",
         logs="https://worldchain-mainnet.g.alchemy.com/public", window=100),
    dict(id=1313161554, path="aurora", name="Aurora",
         rpc="https://mainnet.aurora.dev", logs="https://mainnet.aurora.dev", window=5000),
    dict(id=204, path="opbnb", name="opBNB",
         rpc="https://opbnb-rpc.publicnode.com", logs="https://opbnb-mainnet-rpc.bnbchain.org", window=5000),
    dict(id=8217, path="kaia", name="Kaia",
         rpc="https://public-en.node.kaia.io", logs="https://public-en.node.kaia.io", window=5000),
    dict(id=196, path="okb", name="X Layer",
         rpc="https://rpc.xlayer.tech", logs="https://rpc.xlayer.tech", window=100),
    dict(id=999, path="hyper", name="HyperEVM",
         rpc="https://rpc.hyperliquid.xyz/evm", logs="https://rpc.hyperliquid.xyz/evm", window=1000),
    dict(id=143, path="monad", name="Monad",
         rpc="https://rpc.monad.xyz", logs="https://rpc.monad.xyz", window=100),
    dict(id=9745, path="plasma", name="Plasma",
         rpc="https://rpc.plasma.to", logs="https://rpc.plasma.to", window=5000),
    dict(id=747474, path="katana", name="Katana",
         rpc="https://rpc.katana.network", logs="https://rpc.katana.network", window=5000),
    dict(id=43111, path="hemi", name="Hemi",
         rpc="https://rpc.hemi.network/rpc", logs="https://rpc.hemi.network/rpc", window=5000),
    dict(id=4326, path="mega", name="MegaETH",
         rpc="https://mainnet.megaeth.com/rpc", logs="https://mainnet.megaeth.com/rpc", window=5000),
    dict(id=2818, path="morph", name="Morph",
         rpc="https://rpc.morphl2.io", logs="https://rpc.morphl2.io", window=5000),
    dict(id=3338, path="peaq", name="peaq",
         rpc="https://peaq.api.onfinality.io/public", logs="https://peaq.api.onfinality.io/public", window=5000),
]

SAFE_API = "https://api.safe.global/tx-service/{path}/api"

# keccak256("ExecutionSuccess(bytes32,uint256)"), emitted by every Safe since
# v1.1.1 when a multisig transaction executes. Sampling its emitters samples the
# Safes that are actually in use, rather than ones somebody chose.
EXECUTION_SUCCESS = "0x442e715f626346e8c54381002da614f62bee8d27386535b2521ec8540898556e"
