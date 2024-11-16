/* exported gapiLoaded */
/* exported gisLoaded */
/* exported handleAuthClick */
/* exported handleSignoutClick */

// TODO(developer): Set to client ID and API key from the Developer Console
const CLIENT_ID = '676716570062-slvgp5lt1edkembl0752kjrd9f6l7fou.apps.googleusercontent.com';
const API_KEY = 'AIzaSyBFKIxYXUDQaOI-Dasanaiv70KXmf-LVKM';

// Discovery doc URL for APIs used by the quickstart
const DISCOVERY_DOC = 'https://sheets.googleapis.com/$discovery/rest?version=v4';

// Authorization scopes required by the API; multiple scopes can be
// included, separated by spaces.
const SCOPES = 'https://www.googleapis.com/auth/spreadsheets';

let tokenClient;
let gapiInited = false;
let gisInited = false;


/**
 * Callback after api.js is loaded.
 */
function gapiLoaded() {
    gapi.load('client', initializeGapiClient);
}

/**
 * Callback after the API client is loaded. Loads the
 * discovery doc to initialize the API.
 */
async function initializeGapiClient() {
    await gapi.client.init({
        apiKey: API_KEY,
        discoveryDocs: [DISCOVERY_DOC],
    });
    gapiInited = true;
}

/**
 * Callback after Google Identity Services are loaded.
 */
function gisLoaded() {
	tokenClient = google.accounts.oauth2.initTokenClient({
		client_id: CLIENT_ID,
		scope: SCOPES,
		callback: '', // defined later
	});
	gisInited = true;
}

/**
 * Enables user interaction after all libraries are loaded.
 */

export function initializeGAPI(){
	gapiLoaded();
	gisLoaded();	
}

/**
 *  Sign in the user upon button click.
 */
async function handleAuth() {
	return new Promise((resolve, reject) => {
		tokenClient.callback = async (resp) => {
			if (resp.error !== undefined) {
				reject(resp);
			}
			resolve();
		};

		if (gapi.client.getToken() === null) {
			// Prompt the user to select a Google Account and ask for consent to share their data
			// when establishing a new session.
			tokenClient.requestAccessToken({prompt: 'consent'});
		} else {
			// Skip display of account chooser and consent dialog for an existing session.
			tokenClient.requestAccessToken({prompt: ''});
		}
	});
}

/**
 *  Sign out the user upon button click.
 */
function handleSignoutClick() {
	const token = gapi.client.getToken();
	if (token !== null) {
		google.accounts.oauth2.revoke(token.access_token);
		gapi.client.setToken('');
	}
}

async function loadAllUsers(){
	let response;
	try {
		// Fetch first 10 files
		response = await gapi.client.sheets.spreadsheets.values.get({
			spreadsheetId: '1fTL7G5VZjO26biDB4BdcffQOcy8G9cUmhBonKOuZFdA',
			range: 'Игроки!A2:S',
		});
	} catch (err) {
		console.error(err);
		return;
	}

	const range = response.result;
	if (!range || !range.values || range.values.length == 0) {
		console.error("No values found.");
		return;
	}

	return range.values;
}

export async function loadAllUsersAsync(){
	await handleAuth();	
	return await loadAllUsers();
}