import ActionStatusInfoComponent from "./action-status-info-component.riot";
import * as riot from 'riot';

function triggerInfo(message) {
    let infoBox = document.getElementById("info-box");
    if (!infoBox) {
        const el = document.getElementById("root");
        infoBox = document.createElement("div");
        infoBox.id = "info-box";
        el.insertBefore(infoBox, el.firstChild);
    }
    let comp = riot.mount(infoBox, ActionStatusInfoComponent, 'action-status-info-component')[0];
    comp.showInfo(message);
}

export default { triggerInfo };