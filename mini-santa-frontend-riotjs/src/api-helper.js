import AuthHelper from './auth-helper.js';

const baseUrl = AuthHelper.apiBaseUrl();

async function getId() {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/users/my_id`);
}

async function getNickname() {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/users/my_nickname`);
}

async function getPools() {
    return await AuthHelper.sendRequestWithStatusHandler(`${baseUrl}/api/santa/pools`);
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

export default { getId, getNickname, getPools, createPool };