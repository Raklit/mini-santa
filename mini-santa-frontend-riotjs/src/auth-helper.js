import InfoHandler from "./info-handler.js";
import { router } from "@riotjs/route";

function apiBaseUrl() {
    return "http://localhost:8080";
}

const baseUrl = apiBaseUrl();

async function loginByPassword(login, password) {
    const body = {
        'grant_type': 'password',
        'username': login,
        'password': password,
        'client_id': 'api'
    };
    
    const headers = new Map();
    headers.set('Content-Type', 'application/x-www-form-urlencoded');

    const params = {
        method: 'POST',
        headers: headers,
        body: new URLSearchParams(body).toString()
    };

    const response = await fetch(`${baseUrl}/oauth/token`, params);
    const resp_json = await response.json();

    if (!resp_json || !resp_json["access_token"] || !resp_json["refresh_token"] || !resp_json["expires_in"]) { return; }
    
    let expires = new Date();
    expires.setSeconds(expires.getSeconds() + resp_json["expires_in"]);
    localStorage.setItem('refresh_token', resp_json["refresh_token"]);
    localStorage.setItem('access_token', resp_json["access_token"]);
    localStorage.setItem('expires', expires);
}


async function refreshTokens() {
    const refreshToken = localStorage.getItem('refresh_token');
    const body = {
        'grant_type': 'refresh_token',
        'refresh_token': refreshToken,
        'client_id': 'api'
    };

    const headers = new Map();
    headers.set('Content-Type', 'application/x-www-form-urlencoded');

    const params = {
        method: 'POST',
        headers: headers,
        body: new URLSearchParams(body).toString()
    };

    const response = await fetch(`${baseUrl}/oauth/token`, params);
    const resp_json = await response.json();
    
    if (!resp_json || !resp_json["access_token"] || !resp_json["refresh_token"] || !resp_json["expires_in"]) { return; }
    
    let expires = new Date();
    expires.setSeconds(expires.getSeconds() + resp_json["expires_in"]);
    localStorage.setItem('refresh_token', resp_json["refresh_token"]);
    localStorage.setItem('access_token', resp_json["access_token"]);
    localStorage.setItem('expires', expires);
}

async function logout() {
    localStorage.removeItem("refresh_token");
    localStorage.removeItem("access_token");
    localStorage.removeItem("expires");
}

async function signup(login, password, confirmPassword, nickname, email, inviteCode) {
    let body = {
        "login" : login,
        "password" : password,
        "confirm_password" : confirmPassword,
        "nickname" : nickname,
        "email" : email,
        "invite_code" : inviteCode
    };
    let params = {method: "POST", headers: {'Content-Type': 'application/json'}, body: JSON.stringify(body)};
    return await sendRequestWithStatusHandler(`${baseUrl}/api/sign_up`, params);

}

async function getAccessToken() {
    const needLogin = await goToLoginPageIfNeed();
    if (needLogin) {
        return null;
    }
    const now = new Date();
    const expires = localStorage.getItem('expires');
    if (expires <= now) {
        await refreshTokens();
    }
    return localStorage.getItem('access_token');
}

async function goToLoginPageIfNeed() {
    const accessToken = localStorage.getItem('access_token');
    const refreshToken = localStorage.getItem('refresh_token');
    const expires = localStorage.getItem('expires');
    if (!accessToken || !refreshToken || !expires) {
        router.push("/login");
        return true;
    }
    return false;
}

async function sendRequest(url, params = {method: 'GET', headers: new Map()}) {
    while (params.headers.has('Authorization')) {
        params.headers.removeItem('Authorization');
    }
    const accessToken = await getAccessToken();
    params.headers.set('Authorization', `Bearer ${accessToken}`);
    return await fetch(url, params);
}

async function sendRequestWithStatusHandler(url, params = {method: 'GET', headers: new Map()}, showResultOnOk = false) {
    try {
        const response = await sendRequest(url, params);
        const resp_json = await response.json();
        if (showResultOnOk || resp_json.status != 'OK') {
            InfoHandler.triggerInfo(JSON.stringify(resp_json));
        }
        return resp_json;
    } catch (error) {
        InfoHandler.triggerInfo("Network error while sending request");
        console.error(error.message);
    }
    return null;
}


export default { apiBaseUrl, loginByPassword, refreshTokens, getAccessToken, logout, signup, sendRequest, sendRequestWithStatusHandler };

