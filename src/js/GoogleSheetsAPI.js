/* exported gapiLoaded */
/* exported gisLoaded */
/* exported handleAuthClick */
/* exported handleSignoutClick */

// TODO(developer): Set to client ID and API key from the Developer Console
const CLIENT_ID =
  "676716570062-slvgp5lt1edkembl0752kjrd9f6l7fou.apps.googleusercontent.com";
const API_KEY = "AIzaSyBFKIxYXUDQaOI-Dasanaiv70KXmf-LVKM";

const TOKEN_STORAGE_KEY = "google-sheets-token";

// Discovery doc URL for APIs used by the quickstart
const DISCOVERY_DOC =
  "https://sheets.googleapis.com/$discovery/rest?version=v4";

// Authorization scopes required by the API; multiple scopes can be
// included, separated by spaces.
const SCOPES = "https://www.googleapis.com/auth/spreadsheets";

const SPREADSHEET_ID = "1fTL7G5VZjO26biDB4BdcffQOcy8G9cUmhBonKOuZFdA";

let tokenClient;
let gapiInited = false;
let gisInited = false;

/**
 * Callback after api.js is loaded.
 */
async function gapiLoaded() {
  return new Promise((resolve, reject) => {
    gapi.load("client", resolve);
  });
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
    callback: "", // defined later
  });
  gisInited = true;
}

/**
 * Enables user interaction after all libraries are loaded.
 */

export async function initializeGAPI() {
  await gapiLoaded();
  await initializeGapiClient();
  gisLoaded();

  return await checkStoredToken();
}

async function checkStoredToken() {
  if (gapi.client.getToken() !== null) return true;

  let storedToken = localStorage.getItem(TOKEN_STORAGE_KEY);

  if (storedToken) {
    const token = JSON.parse(storedToken);

    // Validate token by making a request
    try {
      gapi.client.setToken(token);

      await gapi.client.sheets.spreadsheets.get({
        spreadsheetId: SPREADSHEET_ID, // Use an actual, accessible spreadsheet ID
      });

      return true;
    } catch (err) {
      console.error("Token validation failed", err);
      localStorage.removeItem(TOKEN_STORAGE_KEY);
    }
  }

  return false;
}

/**
 *  Sign in the user upon button click.
 */
async function handleAuth() {
  return new Promise(async (resolve, reject) => {
    tokenClient.callback = async (resp) => {
      if (resp.error !== undefined) {
        localStorage.removeItem(TOKEN_STORAGE_KEY);
        reject(resp);
        return;
      }

      localStorage.setItem(
        TOKEN_STORAGE_KEY,
        JSON.stringify(gapi.client.getToken())
      );
      resolve();
    };

    if (await checkStoredToken()) {
      resolve();
    } else {
      tokenClient.requestAccessToken({ prompt: "" }); // Request a new token if invalid
    }
  });
}

export async function handleSigninClick() {
  await handleAuth();

  return await checkStoredToken();
}

/**
 *  Sign out the user upon button click.
 */
export function handleSignoutClick() {
  const token = gapi.client.getToken();
  if (token !== null) {
    google.accounts.oauth2.revoke(token.access_token);
    gapi.client.setToken("");
    localStorage.removeItem(TOKEN_STORAGE_KEY);
  }
}

export async function loadAllUsers() {
  await handleAuth();

  let response;
  try {
    // Fetch all files
    response = await gapi.client.sheets.spreadsheets.values.get({
      spreadsheetId: SPREADSHEET_ID,
      range: "Игроки!A2:S",
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

export async function createNewUser(id, name) {
  await handleAuth();

  const values = [
    [`'${id}`, String(name), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  ];

  try {
    // Use the Google Sheets API to append the new row
    const response = await gapi.client.sheets.spreadsheets.values.append({
      spreadsheetId: SPREADSHEET_ID,
      range: "Игроки", // Append to the 'Игроки' sheet
      valueInputOption: "USER_ENTERED", // Values are not parsed, they are input as they are
      resource: {
        values,
      },
    });

    const range = response.result.updates.updatedRange;
    const row = range.split(/\D+/im)[1];

    values[0][2] = `=D${row}+N${row}`;
    values[0][3] = `='Бальная система'!$B$2*E${row}+'Бальная система'!$B$3*F${row}+'Бальная система'!$B$10*M${row}+'Бальная система'!$B$9*L${row}+'Бальная система'!$B$8*K${row}+'Бальная система'!$B$7*J${row}+'Бальная система'!$B$6*I${row}+'Бальная система'!$B$5*G${row}+'Бальная система'!$B$4*H${row}`;
    values[0][13] = `='Бальная система'!$B$2*O${row}+'Бальная система'!$B$3*P${row}+'Бальная система'!$B$10*S${row}+'Бальная система'!$B$5*Q${row}+'Бальная система'!$B$4*R${row}`;

    const response2 = await gapi.client.sheets.spreadsheets.values.update({
      spreadsheetId: SPREADSHEET_ID,
      range, // Append to the 'Игроки' sheet
      valueInputOption: "USER_ENTERED", // Values are not parsed, they are input as they are
      resource: {
        values,
      },
    });

    console.log("Successfully added new user:", response2);
  } catch (err) {
    console.error("Error adding new user:", err);
  }
}

// {name: "Игрок 1", id: "001", role: "Мафия", score: 0, winner: bool, rounds: ["", "", "", "", ""]}
export async function createNewGameLog(users) {
  await handleAuth();

  try {
    // Use the Google Sheets API to append the new row
    const response = await gapi.client.sheets.spreadsheets.get({
      spreadsheetId: SPREADSHEET_ID,
    });

    const sheets = response.result.sheets;

    // calculate last game ID with name 'Игра N'
    let lastGameId = 1;
    for (let sheet of sheets) {
      const title = sheet.properties.title;
      if (title.includes("Игра")) {
        const gameId = parseInt(title.split(" ")[1]);
        if (gameId > lastGameId) {
          lastGameId = gameId;
        }
      }
    }

    const newGameId = lastGameId + 1;
    const newSheetTitle = `Игра ${newGameId}`;

    // calculate max round numbers from users array
    const maxRounds = Math.max(...users.map((user) => user.rounds.length));

    const rounds = [];
    for (let i = 0; i < maxRounds; i++) {
      rounds.push(`${i + 1}`);
    }

    const values = [["#", "Игрок", "Роль", "Баллов", "", ...rounds]];

    for (let i = 0; i < users.length; i++) {
      const user = users[i];
      values.push([
        `'${user.id}`,
        user.name,
        user.role,
        user.score,
        (user.winner ? "🏆" : "") + (user.best_player ? "⭐" : ""),
        ...user.rounds,
      ]);
    }

    const response3 = await gapi.client.sheets.spreadsheets.batchUpdate({
      spreadsheetId: SPREADSHEET_ID,
      resource: {
        requests: [
          {
            addSheet: {
              properties: {
                title: newSheetTitle,
              },
            },
          },
        ],
      },
    });

    const response2 = await gapi.client.sheets.spreadsheets.values.update({
      spreadsheetId: SPREADSHEET_ID,
      range: `${newSheetTitle}!A1:Z`,
      valueInputOption: "USER_ENTERED", // Values are not parsed, they are input as they are
      resource: {
        values,
      },
    });

    // Make header styles
    const headerStyle = {
      backgroundColor: {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
      },
      foregroundColor: {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
      },
      bold: true,
    };

    const sheetId = response3.result.replies[0].addSheet.properties.sheetId;

    const response4 = await gapi.client.sheets.spreadsheets.batchUpdate({
      spreadsheetId: SPREADSHEET_ID,
      resource: {
        requests: [
          {
            repeatCell: {
              range: {
                sheetId,
                startRowIndex: 0,
                endRowIndex: 1,
              },
              cell: {
                userEnteredFormat: {
                  backgroundColor: headerStyle.backgroundColor,
                  horizontalAlignment: "CENTER",
                  textFormat: {
                    foregroundColor: headerStyle.foregroundColor,
                    bold: headerStyle.bold,
                  },
                },
              },
              fields:
                "userEnteredFormat(backgroundColor,textFormat,horizontalAlignment)",
            },
          },
          {
            updateSheetProperties: {
              properties: {
                sheetId,
                gridProperties: {
                  frozenRowCount: 1,
                },
              },
              fields: "gridProperties.frozenRowCount",
            },
          },
          {
            autoResizeDimensions: {
              dimensions: {
                sheetId,
                dimension: "COLUMNS",
                startIndex: 0,
                endIndex: 100,
              },
            },
          },
          {
            updateDimensionProperties: {
              range: {
                sheetId,
                dimension: "COLUMNS",
                startIndex: 4,
                endIndex: 100,
              },
              properties: {
                pixelSize: 30,
              },
              fields: "*",
            },
          },
        ],
      },
    });

    // Fetch all files
    const response5 = await gapi.client.sheets.spreadsheets.values.get({
      spreadsheetId: SPREADSHEET_ID,
      range: "Игроки!A2:S",
      valueRenderOption: "FORMULA",
    });

    const range = response5.result;
    if (!range || !range.values || range.values.length == 0) {
      console.error("No values found.");
      return;
    }

    // Update user scores
    const userValues = range.values;
    for (const userSheet of userValues) {
      const userId = userSheet[0].replace("'", "");

      const user = users.find((user) => user.id === userId);
      if (!user) continue;

      // update 4th column with new game count
      userSheet[4] = userSheet[4] * 1 + 1;
      if (user.winner) {
        // update 5th column with new game winner
        userSheet[5] = userSheet[5] * 1 + 1;
        // update Nth column with new game winner
        userSheet[user.role_index] = userSheet[user.role_index] * 1 + 1;
      }
    }

    const response6 = await gapi.client.sheets.spreadsheets.values.update({
      spreadsheetId: SPREADSHEET_ID,
      range: "Игроки!A2:S",
      valueInputOption: "USER_ENTERED", // Values are not parsed, they are input as they are
      resource: {
        values: userValues,
      },
    });

    console.log("Successfully added new user:", response6);
  } catch (err) {
    console.error("Error adding new user:", err);
  }
}
