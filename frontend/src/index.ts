import { isAllowed, setAllowed, getUserInfo } from "@stellar/freighter-api";
import { Contract, networks } from "eras-tour-nft-client";

// Related to Contract
// Replace with the value you get when running
// "soroban config identity address swift"
const admin = "GB6LZJYBWT5VTYSDAVXNQN6PAXITAZVTJSKDMRELPDQ3YYHIXZJF4RQT";

// freighter related HTML elements
const freighterWrapper = document.querySelector("#freighter-wrap");
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

async function checkSeatOwner({ seat_num }: { seat_num: number }) {
  const ownerOfTx = await erasTourContract.ownerOf({
    seat_num,
  });

  try {
    const ownerOfTxResult = await ownerOfTx.simulate();
    console.log("**LOG** ownerOfTxResult: ", ownerOfTxResult.result);
  } catch (e) {
    console.log("this seat is not taken by anyone");
  }
}

async function mint_seat() {
  console.log("mint_seat");
  const loggedInUserPubKey = await getPubKey();

  console.log("**LOG** loggedInUserPubKey: ", loggedInUserPubKey);
  console.log("**LOG** seat_number: ", seat_number);
  console.log("**LOG** hasContractInit: ", hasContractInit);

  if (loggedInUserPubKey && seat_number) {
    if (hasContractInit) {
      console.log("**LOG** erasTourContract.mint");
      await erasTourContract.mint({
        to: loggedInUserPubKey,
        seat_num: seat_number,
      });
    }
  }
}

async function onSelectSeat() {
  let previous_seat: any;
  let previous_seat_color: any;
  let seats = document.querySelectorAll(`[id^="seat-"]`);
  let seat_info = document.querySelector(".info") as HTMLElement;
  let seat_number_text = document.getElementById("seat");

  seats.forEach((seat) => {
    seat.addEventListener("click", function (e) {
      const target = e.target as HTMLTextAreaElement;
      let previous_color = target.style.fill;
      let selected_seat_num = target.id.split("-")[1];
      seat_number = parseInt(selected_seat_num);
      seat_number_text.innerHTML = selected_seat_num;

      checkSeatOwner({ seat_num: seat_number });

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

async function init() {
  if (!hasContractInit) {
    await erasTourContract.initialize({
      admin,
      name: "Eras Tour",
      symbol: "eras",
    });

    hasContractInit = true;
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
}

init();
onSelectSeat();
mintButton.addEventListener("click", async () => {
  await mint_seat();
});
