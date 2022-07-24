// Required imports
import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/api';

const WEB_SOCKET = 'ws://127.0.0.1:9944';

/* Sleep Thread */
const sleep = (ms: any) => new Promise(resolve => setTimeout(resolve, ms));

/* Make Connection */
const makeConnection = async () =>
{
    const provider = new WsProvider(WEB_SOCKET);
    const api = await ApiPromise.create({ provider, types: {} });
    await api.isReady;
    return api;
}

/* Retrive Chain Information */
const retrieveChainInfo = async (api: ApiPromise) =>
{
    const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version()
      ]);
    
    console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
}

/* Subscribe Alice Balances */
const subscribeAliceBalance = async (api: ApiPromise) =>
{
    const keyring = new Keyring({ type: 'sr25519'});
    const alice = keyring.addFromUri('//Alice');
    await api.query.system.account(alice.address, (aliceAcct: any) => {
        console.log("Subscribe Alice Account");
        const aliceFreeSub = aliceAcct.data.free;
        console.log(`Alice Account (sub): ${aliceFreeSub}`);
    });
};

/* Subscribe All Runtime Events */
const subscribeRuntimeEvents = async (api: ApiPromise) =>
{
    api.query.system.events((events: any) => 
    {
        console.log(`\nReceived ${events.length} events:`);

        events.forEach((record: any) => 
        {
            const { event, phase } = record;
            const types = event.typeDef;
            
            console.log(`\t${event.section}:${event.method}:: (phase=${phase.toString()})`);
            
            event.data.forEach((data: any, index: any) => 
            {
                console.log(`\t\t\t${types[index].type}: ${data.toString()}`);
            });
        });
    });
}


async function main () 
{

    const api = await makeConnection();

    retrieveChainInfo(api);

    // await subscribeAliceBalance(api);
    
    await subscribeRuntimeEvents(api);
    
    await sleep(600000);
}

main().catch(console.error).finally(() => process.exit());