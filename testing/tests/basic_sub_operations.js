const assert = require('assert');
const eoslime = require('./../eoslime').init();

const TOKEN_WASM_PATH = '../../example/datamkt_gc_opt.wasm';
const TOKEN_ABI_PATH =  '../../example/datamkt.abi.json';

describe('Basic sub operations', function () {

    // Increase mocha(testing framework) time, otherwise tests fails
    this.timeout(15000);

    let contract;
    let tokensIssuer;
    let tokensHolder;
    let sub_id;
    let sub_name;
    let owner;
    var snooze_ms = 300;

    // We call this at the top of each test case, otherwise nodeosd could
    // throw duplication errors (ie, data races).
    const snooze = ms => new Promise(resolve => setTimeout(resolve, ms));

    before(async () => {
        /*
            Accounts loader generates random accounts for easier testing
            But you could use it also to create new accounts on each network
            For more details, visit the documentation
        */
        //let accounts = await eoslime.Account.createRandoms(2);
        //tokensIssuer = accounts[0];
        //tokensHolder = accounts[1];
    });

    beforeEach(async () => {
        /*
            CleanDeployer creates for you a new account behind the scene
            on which the contract code is deployed

            Note! CleanDeployer always deploy the contract code on a new fresh account

            You can access the contract account as -> contract.executor
        */
        contract = await eoslime.CleanDeployer.deploy(TOKEN_WASM_PATH, TOKEN_ABI_PATH);
        await contract.createsub(contract.executor.name, "datamkt");

        let subprofile_tbl = await contract.provider.eos.getTableRows({
            code: contract.name,
            scope: contract.name,
            table: "subprofile",
            json: true
        });

        sub_id = subprofile_tbl["rows"][0]["sub_id"];
        sub_name = subprofile_tbl["rows"][0]["sub_name"];
        owner = subprofile_tbl["rows"][0]["owner"];

        assert.equal(sub_id, 0, "Wrong sub id.");
        assert.equal(sub_name, "datamkt", "Wrong sub name.");
        assert.equal(owner, contract.executor.name, "Wrong sub owner.");

    });

    it("An account shouldn't be able to own two subs with the same name", async () => {
        await snooze(snooze_ms);

        var err_json;
        var err_code;
        var eosio_err_code;
        var eosio_err_name;

        try {
            await contract.createsub(contract.executor.name, "datamkt");
        } catch (error) {
                err_json = JSON.parse(error);
                err_code = err_json.code;
                if (err_code == 500) {
                    eosio_err_code = err_json.error.code;
                    eosio_err_name = err_json.error.name;
                }
        }

        // If we get a 409, it likely means the two contract calls were seen as duplicates
        // by nodeos. Basically a data race. Consider sleeping for a few ms in between
        // contract calls. Search the web for "nodejs async sleep".
        assert.equal(err_code, 500, "Instead of an Internal Appliation Error, we got: " + err_json);
        assert.equal(eosio_err_code, 3050003, "Two subs with same name are owned by a single account.");
        assert.equal(eosio_err_name, "eosio_assert_message_exception", "Two subs with same name are owned by a single account.");
    });
});
