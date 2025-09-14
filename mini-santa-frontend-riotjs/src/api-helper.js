import AuthHelper from './auth-helper.js';

const baseUrl = AuthHelper.apiBaseUrl();

const apiBaseUrl = AuthHelper.apiBaseUrl;

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

async function getId() {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/users/my_id`);
}

async function getNickname() {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/users/my_nickname`);
}

async function getPools() {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/santa/pools`);
}

async function getPool(id) {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/santa/pools/id/${id}`)
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
    return AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/santa/pools`, params, true);
}

export default { apiBaseUrl, poolState, getPoolStateFromNum, getId, getNickname, getPool, getPools, createPool };