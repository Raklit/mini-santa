import AuthHelper from './auth-helper.js';

const baseUrl = AuthHelper.apiBaseUrl();

const apiBaseUrl = AuthHelper.apiBaseUrl;
const getAccessToken = AuthHelper.getAccessToken;
const amIInSystem = AuthHelper.amIInSystem;

function poolState() {
    return {
        Created: 0,
        Open: 1,
        Pooling: 2,
        Started: 3,
        Ended: 4
    };
}
const PoolState = poolState();

function getPoolStateFromNum(num) {;
    const stateStrings = Object.keys(PoolState).reduce((acc, key) => {
        acc[PoolState[key]] = key;
        return acc;
    }, {});
    return stateStrings[num] || 'Unknown State';
}

function roomState() {
    return {
        ChoosingAGift: 0,
        BuyingAGift: 1,
        MailerAwaitingGiftDelivery: 2,
        GiftDeliveredToMailer: 3,
        MailerSendGiftToRecipient: 4,
        GiftInAWayToRecipient: 5,
        GiftHasBeenDeliveredToRecipient: 6,
        RecipientTookTheGift: 7
    };
}
const RoomState = roomState();

function getRoomStateFromNum(num) {
    const stateStrings = Object.keys(RoomState).reduce((acc, key) => {
        acc[RoomState[key]] = key;
        return acc;
    }, {});
    return stateStrings[num] || 'Unknown State';
}


async function getId() {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/users/my_id`);
}

async function getNickname() {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/users/my_nickname`);
}

async function amIAdmin() {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/users/am_i_admin`);
}

async function createInviteCode(inviteCode, oneUse) {
    const body = {
        "invite_code" : inviteCode, "one_use" : oneUse,
    };

    const headers = new Map();
    headers.set('Content-Type', 'application/json');

    const params = {
        method: 'POST',
        headers: headers,
        body: JSON.stringify(body)
    };
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/invites`, params, true);
}


async function getInviteCodes() {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/invites`);
}

async function getInviteCode(id) {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/invites/id/${id}`);
}

async function deleteInviteCode(id) {
    const headers = new Map();
    headers.set('Content-Type', 'application/json');

    const params = {
        method: 'DELETE',
        headers: headers,
        body: ""
    };
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/invites/id/${id}`, params, true);
}


async function getPools() {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/santa/pools`);
}

async function getPool(id) {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/santa/pools/id/${id}`);
}

async function deletePool(id) {
    const headers = new Map();
    headers.set('Content-Type', 'application/json');

    const params = {
        method: 'DELETE',
        headers: headers,
        body: ""
    };
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/santa/pools/id/${id}/remove_pool`, params, true);
}

async function amIPoolOwner(id) {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/santa/pools/id/${id}/am_i_resource_owner`);
}

async function getPoolMemberNicknames(id) {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/santa/pools/id/${id}/members`);
}

async function pushPoolState(id) {
    const headers = new Map();
    headers.set('Content-Type', 'application/json');

    const params = {
        method: 'POST',
        headers: headers,
        body: ""
    };
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/santa/pools/id/${id}/push_state`, params);
}

async function createPool(name, description, minPrice, maxPrice) {
    const body = {
        "name" : name, "description" : description, 
        "min_price" : minPrice, "max_price" : maxPrice
    };

    const headers = new Map();
    headers.set('Content-Type', 'application/json');

    const params = {
        method: 'POST',
        headers: headers,
        body: JSON.stringify(body)
    };
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/santa/pools`, params, true);
}

async function addToPool(pool_id, wishlist) {
    const body = {
        "pool_id" : pool_id, "wishlist" : wishlist
    };

    const headers = new Map();
    headers.set('Content-Type', 'application/json');

    const params = {
        method: 'POST',
        headers: headers,
        body: JSON.stringify(body)
    };
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/santa/members`, params, true);
}

async function removeUserFromPool(pool_id, account_id) {
    const headers = new Map();
    headers.set('Content-Type', 'application/json');

    const params = {
        method: 'DELETE',
        headers: headers,
        body: ""
    };
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/santa/pools/id/${pool_id}/remove_member/${account_id}`, params, true);
}

async function removeCurrentUserFromPool(pool_id) {
    const headers = new Map();
    headers.set('Content-Type', 'application/json');

    const params = {
        method: 'DELETE',
        headers: headers,
        body: ""
    };
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/santa/pools/id/${pool_id}/remove_me`, params, true);
}

async function getRooms() {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/santa/rooms/my_rooms`);
}

async function getRoom(id) {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/santa/rooms/id/${id}/info`);
}

async function getLastMessagesInRoom(id) {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/santa/rooms/id/${id}/last_messages`);
    
}

async function sendMessage(room_id, text_content) {
    const body = {
        "text_content" : text_content
    };

    const headers = new Map();
    headers.set('Content-Type', 'application/json');

    const params = {
        method: 'POST',
        headers: headers,
        body: JSON.stringify(body)
    };
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/santa/rooms/id/${room_id}/send_message`, params, false);
}

async function signOutFromAll() {
    const headers = new Map();
    headers.set('Content-Type', 'application/json');

    const params = {
        method: 'DELETE',
        headers: headers,
        body: ""
    };
    let resp_json = await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/users/sign_out_from_all`, params);
    await AuthHelper.logout();
    return resp_json;
}

export default { apiBaseUrl, amIInSystem, getAccessToken, poolState, roomState, getPoolStateFromNum, getRoomStateFromNum, createInviteCode, getInviteCode, getInviteCodes, deleteInviteCode, getId, getNickname, amIAdmin, amIPoolOwner, getPool, getPools, deletePool, getPoolMemberNicknames, createPool, pushPoolState, addToPool, removeUserFromPool, removeCurrentUserFromPool, getRoom, getRooms, getLastMessagesInRoom, sendMessage, signOutFromAll };