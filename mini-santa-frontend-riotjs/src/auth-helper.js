import ClientOAuth2 from "client-oauth2";
import InfoHandler from "./info-handler.js";

function getClient(scopes = ["read, write"]) {
    return new ClientOAuth2({
            clientId: 'api',
            accessTokenUri: 'http://localhost:8080/oauth/token',
            authorizationUri: 'http://localhost:8080/login',
            redirectUri: 'http://localhost:8080/',
            scopes: scopes
        });
}

async function loginByPassword(login, password) {
    const authClient = getClient();
    const response = await authClient.owner.getToken(login, password);
    if (!response || !response.accessToken || !response.refreshToken || !response.expires) { return; }
    localStorage.setItem('refresh_token', response.refreshToken);
    localStorage.setItem('access_token', response.accessToken);
    localStorage.setItem('expires', response.expires);
}


async function refreshTokens() {
    const authClient = getClient();
    const accessToken = localStorage.getItem('access_token');
    const refreshToken = localStorage.getItem('refresh_token');
    let token = authClient.createToken(accessToken, refreshToken, 'bearer', {data: ''});
    const response = await token.refresh();
    if (!response || !response.accessToken || !response.refreshToken || !response.expires) { return; }
    localStorage.setItem('refresh_token', response.refreshToken);
    localStorage.setItem('access_token', response.accessToken);
    localStorage.setItem('expires', response.expires);
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
    try {
        let response = await fetch('http://localhost:8080/api/sign_up', params);
        let resp_json = await response.json();
        InfoHandler.triggerInfo(resp_json.body);
    } catch (error) {
        InfoHandler.triggerInfo("Network error while sending request");
        console.log(error.message);
    }

}

async function getAccessToken() {
    const now = new Date();
    const expires = localStorage.getItem('expires');
    if (expires <= now) {
        await refreshTokens();
    }
    return localStorage.getItem('access_token');
}

async function sendRequest(url, params = {method: 'GET', headers: new Map()}) {
    while (params.headers.has('Authorization')) {
        params.headers.removeItem('Authorization');
    }
    const accessToken = await getAccessToken();
    params.headers.set('Authorization', `Bearer ${accessToken}`);
    return await fetch(url, params);
}

export default { getClient, loginByPassword, refreshTokens, getAccessToken, logout, signup, sendRequest };

