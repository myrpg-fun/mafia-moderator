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
  if (gapi.client.getToken() === null) {
    let storedToken = localStorage.getItem(TOKEN_STORAGE_KEY);

    if (storedToken) {
      const token = JSON.parse(storedToken);

      gapi.client.setToken(token);
    }
  }

  // Validate token by making a request
  try {
    await gapi.client.sheets.spreadsheets.values.update({
      spreadsheetId: SPREADSHEET_ID,
      range: "Игроки!A1:A1",
      valueInputOption: "USER_ENTERED", // Values are not parsed, they are input as they are
      resource: {
        values: [[]],
      },
    });

    return true;
  } catch (err) {
    console.error("Token validation failed", err);
    localStorage.removeItem(TOKEN_STORAGE_KEY);
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
    return [];
  }

  const values = range.values.filter((user) => user.length === 19);

  return values;
}

function createNewUserRow(id, name, row) {
  return [
    `'${id}`,
    String(name),
    `=D${row}+N${row}`,
    `='Бальная система'!$B$2*E${row}+'Бальная система'!$B$3*F${row}+'Бальная система'!$B$10*M${row}+'Бальная система'!$B$9*VALUE(LEFT(L${row}, FIND("/", L${row}) - 1))+'Бальная система'!$B$8*VALUE(LEFT(K${row}, FIND("/", K${row}) - 1))+'Бальная система'!$B$7*VALUE(LEFT(J${row}, FIND("/", J${row}) - 1))+'Бальная система'!$B$6*VALUE(LEFT(I${row}, FIND("/", I${row}) - 1))+'Бальная система'!$B$5*VALUE(LEFT(G${row}, FIND("/", G${row}) - 1))+'Бальная система'!$B$4*VALUE(LEFT(H${row}, FIND("/", H${row}) - 1))`,
    0,
    0,
    `'0/0`,
    `'0/0`,
    `'0/0`,
    `'0/0`,
    `'0/0`,
    `'0/0`,
    0,
    `='Бальная система'!$B$2*O${row}+'Бальная система'!$B$3*P${row}+'Бальная система'!$B$10*S${row}+'Бальная система'!$B$5*VALUE(LEFT(Q${row}, FIND("/", Q${row}) - 1))+'Бальная система'!$B$4*VALUE(LEFT(R${row}, FIND("/", R${row}) - 1))`,
    0,
    0,
    `'0/0`,
    `'0/0`,
    0,
  ];
}

function refreshShreetUserRow(userArray, row) {
  if (userArray.length !== 19) return;

  userArray[0] = `'${userArray[0].replace("'", "")}`;
  userArray[2] = `=D${row}+N${row}`;
  (userArray[3] = `='Бальная система'!$B$2*E${row}+'Бальная система'!$B$3*F${row}+'Бальная система'!$B$10*M${row}+'Бальная система'!$B$9*VALUE(LEFT(L${row}, FIND("/", L${row}) - 1))+'Бальная система'!$B$8*VALUE(LEFT(K${row}, FIND("/", K${row}) - 1))+'Бальная система'!$B$7*VALUE(LEFT(J${row}, FIND("/", J${row}) - 1))+'Бальная система'!$B$6*VALUE(LEFT(I${row}, FIND("/", I${row}) - 1))+'Бальная система'!$B$5*VALUE(LEFT(G${row}, FIND("/", G${row}) - 1))+'Бальная система'!$B$4*VALUE(LEFT(H${row}, FIND("/", H${row}) - 1))`),
    (userArray[13] = `='Бальная система'!$B$2*O${row}+'Бальная система'!$B$3*P${row}+'Бальная система'!$B$10*S${row}+'Бальная система'!$B$5*VALUE(LEFT(Q${row}, FIND("/", Q${row}) - 1))+'Бальная система'!$B$4*VALUE(LEFT(R${row}, FIND("/", R${row}) - 1))`);
  return userArray;
}

export async function createNewUser(id, name) {
  await handleAuth();

  try {
    const nullValues = [createNewUserRow(id, name, 1)];

    // Use the Google Sheets API to append the new row
    const response = await gapi.client.sheets.spreadsheets.values.append({
      spreadsheetId: SPREADSHEET_ID,
      range: "Игроки", // Append to the 'Игроки' sheet
      valueInputOption: "USER_ENTERED", // Values are not parsed, they are input as they are
      resource: {
        values: nullValues,
      },
    });

    const range = response.result.updates.updatedRange;
    const row = range.split(/\D+/im)[1];

    const values = [createNewUserRow(id, name, row)];

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
export async function createNewGameLog(users, isMafia) {
  await handleAuth();

  try {
    // calculate current date minus 10 hours
    const date = new Date();
    date.setHours(date.getHours() - 10);
    // calculate user stats for current day in format 28 Ноя
    const currentDay = date.getDate();
    const currentMonth = date.toLocaleString("ru", { month: "short" });
    const daySheetTitle = `Итоги ${currentDay} ${currentMonth}`;

    // Use the Google Sheets API to append the new row
    const response = await gapi.client.sheets.spreadsheets.get({
      spreadsheetId: SPREADSHEET_ID,
    });

    const sheets = response.result.sheets;
    const gameNamePrefix = "Игра " + currentDay;

    // calculate last game ID with name 'Игра Day-N'
    let lastGameId = 1;
    for (let sheet of sheets) {
      const title = sheet.properties.title;
      if (title.includes(gameNamePrefix)) {
        const gameId = parseInt(title.split("-")[1]);
        if (gameId >= lastGameId) {
          lastGameId = gameId + 1;
        }
      }
    }

    const newSheetTitle = `${gameNamePrefix}-${lastGameId}`;

    // check if exists sheet with current day
    const daySheet = sheets.find(
      (sheet) => sheet.properties.title === daySheetTitle
    );
    // if not exists create new sheet with current day
    if (!daySheet) {
      const response7 = await gapi.client.sheets.spreadsheets.batchUpdate({
        spreadsheetId: SPREADSHEET_ID,
        resource: {
          requests: [
            {
              addSheet: {
                properties: {
                  title: daySheetTitle,
                },
              },
            },
          ],
        },
      });

      const daySheetId =
        response7.result.replies[0].addSheet.properties.sheetId;

      const response8 = await gapi.client.sheets.spreadsheets.batchUpdate({
        spreadsheetId: SPREADSHEET_ID,
        resource: {
          requests: [
            {
              repeatCell: {
                range: {
                  sheetId: daySheetId,
                  startRowIndex: 0,
                  endRowIndex: 1,
                },
                cell: {
                  userEnteredFormat: {
                    backgroundColor: {
                      red: 0.0,
                      green: 0.0,
                      blue: 0.0,
                    },
                    horizontalAlignment: "CENTER",
                    textFormat: {
                      foregroundColor: {
                        red: 1.0,
                        green: 1.0,
                        blue: 1.0,
                      },
                      bold: true,
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
                  sheetId: daySheetId,
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
                  sheetId: daySheetId,
                  dimension: "COLUMNS",
                  startIndex: 0,
                  endIndex: 100,
                },
              },
            },
            {
              updateDimensionProperties: {
                range: {
                  sheetId: daySheetId,
                  dimension: "COLUMNS",
                  startIndex: 0,
                  endIndex: 1,
                },
                properties: {
                  pixelSize: 40,
                },
                fields: "*",
              },
            },
            {
              updateDimensionProperties: {
                range: {
                  sheetId: daySheetId,
                  dimension: "COLUMNS",
                  startIndex: 4,
                  endIndex: 13,
                },
                properties: {
                  pixelSize: 30,
                },
                fields: "*",
              },
            },
            {
              updateDimensionProperties: {
                range: {
                  sheetId: daySheetId,
                  dimension: "COLUMNS",
                  startIndex: 14,
                  endIndex: 19,
                },
                properties: {
                  pixelSize: 30,
                },
                fields: "*",
              },
            },
            {
              repeatCell: {
                range: {
                  sheetId: daySheetId,
                  startRowIndex: 1,
                  endRowIndex: 100,
                  startColumnIndex: 4,
                  endColumnIndex: 13,
                },
                cell: {
                  userEnteredFormat: {
                    horizontalAlignment: "CENTER",
                  },
                },
                fields: "userEnteredFormat(horizontalAlignment)",
              },
            },
            {
              repeatCell: {
                range: {
                  sheetId: daySheetId,
                  startRowIndex: 1,
                  endRowIndex: 100,
                  startColumnIndex: 14,
                  endColumnIndex: 19,
                },
                cell: {
                  userEnteredFormat: {
                    horizontalAlignment: "CENTER",
                  },
                },
                fields: "userEnteredFormat(horizontalAlignment)",
              },
            },
            {
              repeatCell: {
                range: {
                  sheetId: daySheetId,
                  startRowIndex: 1,
                  endRowIndex: 100,
                  startColumnIndex: 0,
                  endColumnIndex: 1,
                },
                cell: {
                  userEnteredFormat: {
                    horizontalAlignment: "CENTER",
                  },
                },
                fields: "userEnteredFormat(horizontalAlignment)",
              },
            },
          ],
        },
      });

      const values = [
        [
          "№",
          "Имя игрока",
          "Сумма баллов",
          "Баллы Мафии",
          "Игр",
          "🏆",
          "🙂",
          "🔫",
          "🔪",
          "🕵️‍♂️",
          "💋",
          "🚑",
          "⭐",
          "Баллы WW",
          "Игр",
          "🏆",
          "🧑‍🌾",
          "🐺",
          "⭐",
        ],
      ];

      const response9 = await gapi.client.sheets.spreadsheets.values.update({
        spreadsheetId: SPREADSHEET_ID,
        range: `${daySheetTitle}!A1:S`,
        valueInputOption: "USER_ENTERED",
        resource: {
          values,
        },
      });
    }

    // calculate max round numbers from users array
    const maxRounds = Math.max(...users.map((user) => user.rounds.length));

    const rounds = [];
    for (let i = 0; i < maxRounds; i++) {
      rounds.push(`${i + 1}`);
    }

    const values = [["#", "Игрок", "Роль", "", ...rounds]];

    const maxSpacePerRound = new Array(maxRounds).fill(30);
    for (let i = 0; i < maxRounds; i++) {
      let maxSpace = 30;
      for (let y = 0; y < users.length; y++) {
        if (users[y] && users[y].rounds[i]) {
          maxSpace = Math.max(
            maxSpace,
            (Array.from(users[y].rounds[i]).length - 1) * 13 + 25
          );
        }
      }
      maxSpacePerRound[i] = maxSpace;
    }

    for (let i = 0; i < users.length; i++) {
      const user = users[i];
      values.push([
        `'${user.id}`,
        user.name,
        user.role,
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
                  backgroundColor: {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                  },
                  horizontalAlignment: "CENTER",
                  textFormat: {
                    foregroundColor: {
                      red: 1.0,
                      green: 1.0,
                      blue: 1.0,
                    },
                    bold: true,
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
                startIndex: 3,
                endIndex: 4,
              },
              properties: {
                pixelSize: 40,
              },
              fields: "*",
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
          ...maxSpacePerRound.map((maxSpace, index) => ({
            updateDimensionProperties: {
              range: {
                sheetId,
                dimension: "COLUMNS",
                startIndex: 4 + index,
                endIndex: 5 + index,
              },
              properties: {
                pixelSize: maxSpace,
              },
              fields: "*",
            },
          })),
          // centered alighnment for all columns
          {
            repeatCell: {
              range: {
                sheetId,
                startRowIndex: 1,
                endRowIndex: 100,
                startColumnIndex: 3,
                endColumnIndex: 100,
              },
              cell: {
                userEnteredFormat: {
                  horizontalAlignment: "CENTER",
                },
              },
              fields: "userEnteredFormat(horizontalAlignment)",
            },
          },
        ],
      },
    });

    // Update day sheet with new game
    const response5 = await gapi.client.sheets.spreadsheets.values.get({
      spreadsheetId: SPREADSHEET_ID,
      range: `${daySheetTitle}!A2:Z`,
    });

    const daySheetValues = response5.result.values || [];
    if (!daySheetValues) {
      console.error("No values found.");
    }

    // Update user scores or create new scores
    for (let i = 0; i < users.length; i++) {
      const user = users[i];
      const userId = user.id;

      let userSheet = daySheetValues.find((user) => user[0] === userId);

      if (!userSheet) {
        // userId, userName
        userSheet = createNewUserRow(
          userId,
          user.name,
          daySheetValues.length + 2
        );

        daySheetValues.push(userSheet);
      }

      const indexGamesCount = isMafia ? 4 : 14;
      const indexGamesWinner = isMafia ? 5 : 15;
      const indexGamesBestPlayer = isMafia ? 12 : 18;

      try {
        // update 4th column with new game count
        userSheet[indexGamesCount] = userSheet[indexGamesCount] * 1 + 1;
        if (isNaN(userSheet[indexGamesCount])) userSheet[indexGamesCount] = 1;
        if (user.winner) {
          // update 5th column with new game winner
          userSheet[indexGamesWinner] = userSheet[indexGamesWinner] * 1 + 1;
          if (isNaN(userSheet[indexGamesWinner]))
            userSheet[indexGamesWinner] = 1;
        }
      } catch (err) {
        console.error(err);
      }

      try {
        if (user.best_player) {
          // update 12th column with game best player
          userSheet[indexGamesBestPlayer] =
            userSheet[indexGamesBestPlayer] * 1 + 1;
          if (isNaN(userSheet[indexGamesBestPlayer]))
            userSheet[indexGamesBestPlayer] = 1;
        }
      } catch (err) {
        console.error(err);
      }

      // update Nth column with new game winner
      try {
        const roleGames = userSheet[user.role_index]
          .replace("'", "")
          .split("/");
        roleGames[1] = roleGames[1] * 1 + 1;
        if (isNaN(roleGames[1])) roleGames[1] = 1;
        roleGames[0] = roleGames[0] * 1 + user.role_score;
        if (isNaN(roleGames[0])) roleGames[0] = user.role_score;
        userSheet[user.role_index] = "'" + roleGames.join("/");
      } catch (err) {
        console.error(err);
      }
    }

    // update all values ids to 'id
    for (let i = 0; i < daySheetValues.length; i++) {
      refreshShreetUserRow(daySheetValues[i], i + 2);
    }

    // Update day sheet with new user scores
    const response61 = await gapi.client.sheets.spreadsheets.values.update({
      spreadsheetId: SPREADSHEET_ID,
      range: `${daySheetTitle}!A2:S`,
      valueInputOption: "USER_ENTERED", // Values are not parsed, they are input as they are
      resource: {
        values: daySheetValues,
      },
    });

    // Fetch all users
    const response51 = await gapi.client.sheets.spreadsheets.values.get({
      spreadsheetId: SPREADSHEET_ID,
      range: "Игроки!A2:S",
      valueRenderOption: "FORMULA",
    });

    const range = response51.result || [];

    // Update user scores
    const userValues = range.values || [];
    for (let i = 0; i < userValues.length; i++) {
      const userSheet = userValues[i];
      const userId = userSheet[0].replace("'", "");
      refreshShreetUserRow(userSheet, i + 2);

      const user = users.find((user) => user.id === userId);
      if (!user) continue;

      const indexGamesCount = isMafia ? 4 : 14;
      const indexGamesWinner = isMafia ? 5 : 15;
      const indexGamesBestPlayer = isMafia ? 12 : 18;

      try {
        // update 4th column with new game count
        userSheet[indexGamesCount] = userSheet[indexGamesCount] * 1 + 1;
        if (isNaN(userSheet[indexGamesCount])) userSheet[indexGamesCount] = 1;
        if (user.winner) {
          // update 5th column with new game winner
          userSheet[indexGamesWinner] = userSheet[indexGamesWinner] * 1 + 1;
          if (isNaN(userSheet[indexGamesWinner]))
            userSheet[indexGamesWinner] = 1;
        }
      } catch (err) {
        console.error(err);
      }

      try {
        if (user.best_player) {
          // update 12th column with game best player
          userSheet[indexGamesBestPlayer] =
            userSheet[indexGamesBestPlayer] * 1 + 1;
          if (isNaN(userSheet[indexGamesBestPlayer]))
            userSheet[indexGamesBestPlayer] = 1;
        }
      } catch (err) {
        console.error(err);
      }

      // update Nth column with new game winner
      try {
        const roleGames = userSheet[user.role_index]
          .replace("'", "")
          .split("/");
        roleGames[1] = roleGames[1] * 1 + 1;
        if (isNaN(roleGames[1])) roleGames[1] = 1;
        roleGames[0] = roleGames[0] * 1 + user.role_score;
        if (isNaN(roleGames[0])) roleGames[0] = user.role_score;
        userSheet[user.role_index] = "'" + roleGames.join("/");
      } catch (err) {
        console.error(err);
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

    console.log("Successfully updated:", response6);
  } catch (err) {
    console.error("Error adding new user:", err);
  }
}
