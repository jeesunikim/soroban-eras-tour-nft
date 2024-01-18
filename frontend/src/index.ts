import { isAllowed, setAllowed, getUserInfo } from "@stellar/freighter-api";
import { Contract, networks } from "eras-tour-nft-client";

// Contract
const admin = "GD6523U5GOVMLSWC4PCNRURU4QKDR6Q76YDAIJPDJDGYFEJ43GBG7VWK";
// freighter related
const freighterWrapper = document.querySelector("#freighter-wrapWrapper");
const ellipsis = document.querySelector(
  "#freighter-wrap .ellipsis"
) as HTMLElement;
const freighterButton = document.querySelector(
  "[data-connect]"
) as HTMLButtonElement;
let hasContractInit = false;

// contract related
const erasTourContract = new Contract({
  ...networks.testnet,
  rpcUrl: "https://soroban-testnet.stellar.org", // from https://soroban.stellar.org/docs/reference/rpc#public-rpc-providers
});
const mintButton = document.querySelector("#mint");
let seat_number: number;

console.log("**LOG** erasTourContract: ", erasTourContract);
console.log("**LOG** networks: ", networks);

// Freighter
async function getPubKey() {
  const { publicKey } = await getUserInfo();
  return publicKey;
}

async function setLoggedIn() {
  const publicKey = await getPubKey();
  const shortenedPubKey =
    publicKey.substring(0, 4) + "..." + publicKey.substring(52);
  ellipsis.innerHTML = `Signed in as ${shortenedPubKey}`;
  ellipsis.title = publicKey;
}

if (await isAllowed()) {
  if (await getPubKey()) {
    setLoggedIn();
  } else {
    freighterWrapper.innerHTML =
      "Freighter is locked.<br>Sign in & refresh the page.";
  }
} else {
  freighterButton.addEventListener("click", async () => {
    freighterButton.disabled = true;
    await setAllowed();
    await setLoggedIn();
  });
}

function onSelectSeat() {
  let previous_seat: any;
  let previous_seat_color: any;
  let seats = document.querySelectorAll(`[id^="seat-"]`);
  let seat_info = document.querySelector(".info") as HTMLElement;
  let seat_number_text = document.getElementById("seat");

  console.log("**LOG** seat_info: ", seat_info);

  seats.forEach((seat) => {
    seat.addEventListener("click", function (e) {
      const target = e.target as HTMLTextAreaElement;
      let previous_color = target.style.fill;
      let selected_seat_num = target.id.split("-")[1];
      seat_number = parseInt(selected_seat_num);
      seat_number_text.innerHTML = selected_seat_num;

      console.log("**LOG** seat_number_text: ", seat_number_text);

      if (e.target) {
        target.style.fill = "#ff0000";
        target.style.stroke = "#ff0000";
        seat_info.style.display = "block";

        if (previous_seat) {
          previous_seat.style.fill = previous_seat_color;
          previous_seat.style.stroke = previous_seat_color;
        }
      }

      if (previous_seat !== e.target) {
        previous_seat = e.target;
        previous_seat_color = previous_color;
      }
    });
  });
}

async function mint_seat() {
  const loggedInUserPubKey = await getPubKey();

  console.log("**LOG** seat_number: ", seat_number);
  console.log("**LOG** loggedInUserPubKey: ", loggedInUserPubKey);

  if (loggedInUserPubKey && seat_number) {
    if (!hasContractInit) {
      await erasTourContract.initialize({
        admin,
        name: "Eras Tour",
        symbol: "eras",
      });

      hasContractInit = true;
    }

    if (hasContractInit) {
      const hi = await erasTourContract.mint({
        to: loggedInUserPubKey,
        seat_num: seat_number,
      });
      console.log("**LOG** hi: ", hi);
    }
  }
}

onSelectSeat();
mintButton.addEventListener("click", mint_seat);
