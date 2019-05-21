// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const { Config, Scenario } = require("@holochain/holochain-nodejs")
Scenario.setTape(require("tape"))

const dnaPath = "./dist/holochain-point-of-interest.dna.json"
const agentAlice = Config.agent("alice")
const agentBob = Config.agent("bob")
const dna = Config.dna(dnaPath)
const instanceAlice = Config.instance(agentAlice, dna)
const instanceBob = Config.instance(agentBob, dna)
const scenario = new Scenario([instanceAlice, instanceBob], { debugLog: false })

scenario.runTape("point of interest demo tests", async (t, { alice, bob }) => {

    // No points at initial run of app returns empty FeatureCollection
    const result = await alice.callSync("point_of_interest", "get_all_points", {"args":{}})
    // Make sure returns empty geojson FeatureCollection
    t.deepEqual(result, { Ok: {"type":"FeatureCollection", "features":{}} })

    // Make sure that node sees other node points
    var aliceTestPoint = {
            "type": "Feature",
            "properties": {
                "name": "Alice test point"
            },
            "geometry": {
                "type": "Point",
                "coordinates": [-114.5, 34.5]
            }};
    const aliceAddr = await alice.callSync("point_of_interest", "add_point", {"entry" : aliceTestPoint})
    const aliceResult = await alice.callSync("point_of_interest", "get_all_points", {"args":{}})
    t.deepEqual(aliceResult, { Ok: {"type":"FeatureCollection", "features":[aliceTestPoint]} })

    const bobResult = await bob.callSync("point_of_interest", "get_all_points", {"args":{}})
    t.deepEqual(bobResult, { Ok: {"type":"FeatureCollection", "features":[aliceTestPoint]} })
})
