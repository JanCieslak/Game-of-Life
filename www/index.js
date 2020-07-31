import { Universe } from "../pkg";

const pre = document.getElementById("game_of_life_canvas");
const table = document.getElementById("data_table");
const fromStateForm = document.getElementById("fromState");
const toStateForm = document.getElementById("toState");
const withConditionForm = document.getElementById("condition");
const addRuleForm = document.getElementById("addRuleBtn");
const universe = Universe.new();
let rules = [{fromState: "Alive", toState: "Dead", withCondition: "x < 2"},
             {fromState: "Alive", toState: "Alive", withCondition: "(x == 3) || (x == 2)"},
             {fromState: "Alive", toState: "Dead", withCondition: "x > 3"},
             {fromState: "Dead", toState: "Alive", withCondition: "x == 3"}];


const addRule = (fromState, toState, withCondition) => {
    let newRow = table.insertRow();
    let newCell1 = newRow.insertCell(0);
    let newCell2 = newRow.insertCell(1);
    let newCell3 = newRow.insertCell(2);
    newCell1.textContent = fromState;
    newCell2.textContent = toState;
    newCell3.textContent = withCondition;

    newRow.addEventListener('click', (e) => {
        //// remove rule with Universe method
        universe.remove_rule(newRow.rowIndex - 1);
        table.deleteRow(newRow.rowIndex - 1);
    });

    //// add new rule to the wasm Universe field with method
    universe.add_rule(fromState, toState, withCondition);
};

//// render the grid
const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();

    requestAnimationFrame(renderLoop);
};

//// restart the simulation
window.addEventListener('keypress', (event) => {
    if (event.key === "r") {
        universe.restart();
    }
});

//// init table
for (let i = 0; i < rules.length; i++) {
    addRule(rules[i].fromState, rules[i].toState, rules[i].withCondition);
}

//// add rule form init
addRuleForm.addEventListener('click', (event) => {
    console.log(fromStateForm.options[fromStateForm.selectedIndex].text + " "
        + toStateForm.options[toStateForm.selectedIndex].text + " "
        + withConditionForm.value);

    addRule(fromStateForm.options[fromStateForm.selectedIndex].text,
        toStateForm.options[toStateForm.selectedIndex].text,
        withConditionForm.value);
});

renderLoop();