import { SecretNetworkClient, Wallet } from "secretjs";
import * as fs from "fs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC);

const contract_wasm = fs.readFileSync("../contract.wasm");

const secretjs = new SecretNetworkClient({
    chainId: "pulsar-2",
    url: "https://api.pulsar.scrttestnet.com",
    wallet: wallet,
    walletAddress: wallet.address,
});

let codeId = 20855
let contractHash = "1231df80207866864cd3cf2da47c9294e9dcd1f9a8baad3cff5f156c9adbfb27"

let upload_contract = async () => {
    let tx = await secretjs.tx.compute.storeCode(
        {
            sender: wallet.address,
            wasm_byte_code: contract_wasm,
            source: "",
            builder: "",
        },
        {
            gasLimit: 4_000_000,
        }
    );

    const codeId = Number(
        tx.arrayLog.find((log) => log.type === "message" && log.key === "code_id")
            .value
    );

    console.log("codeId: ", codeId);

    const contractCodeHash = (
        await secretjs.query.compute.codeHashByCodeId({ code_id: codeId })
    ).code_hash;
    console.log(`Contract hash: ${contractCodeHash}`);

};
// upload_contract();

let contractAddress = "secret152ez4w3mmzwr288a2vu47txtpkzvdqcpemh2xc"

async function instantiateContract() {
    const initMsg = { entropy: "this is the entropy!! asdfkmnekrln12321234" }
    const tx = await secretjs.tx.compute.instantiateContract(
        {
            sender: wallet.address,
            code_id: codeId,
            code_hash: contractHash, // optional but way faster
            init_msg: initMsg,
            label: "secret business card demo" + Math.ceil(Math.random() * 10000),
            init_funds: [], // optional
        },
        {
            gasLimit: 4_000_000,
        },
    );

    const contractAddress = tx.arrayLog.find(
        (log) => log.type === "message" && log.key === "contract_address",
    ).value;
    console.log(contractAddress)
}

// instantiateContract();

async function createCard() {
    const tx = await secretjs.tx.compute.executeContract(
        {
            sender: wallet.address,
            contract_address: contractAddress,
            code_hash: contractHash, // optional but way faster
            msg: {
                create: {
                    card: {
                        name: "Theo",
                        address: "6969 Cool St",
                        phone: "7032201994"
                    },
                    index: 0
                }
            },
            sent_funds: [], // optional
        },
        {
            gasLimit: 4_000_000,
        },
    );
    console.log(tx)
}

// createCard();

async function createViewingKey() {
    let viewing_key_creation_tx = await secretjs.tx.compute.executeContract(
        {
            sender: wallet.address,
            contract_address: contractAddress,
            code_hash: contractHash,
            msg: {
                generate_viewing_key: {
                    index: 0,
                },
            },
        },
        {
            gasLimit: 4_000_000
        }
    )
    console.log(viewing_key_creation_tx.arrayLog.find(
        log => log.type === "wasm" && log.key === "viewing_key"
    ).value)
}

// createViewingKey();

const viewingKey = "6TmfcXaZYjoP8DBn+I8LvPdjGCzLFZomydWqntoOzIQ="

async function getBusinessCard() {
    const business_card_query_tx = await secretjs.query.compute.queryContract({
        contract_address: contractAddress,
        code_hash: contractHash, // optional but way faster
        query: {
            get_card: {
                wallet: wallet.address,
                viewing_key: viewingKey,
                index: 0,
            }
        },
    })
    console.log(business_card_query_tx)

}

// getBusinessCard()
// spits out:
// {
//   card: { name: 'Theo', address: '6969 Cool St', phone: '7032201994' }
// }

