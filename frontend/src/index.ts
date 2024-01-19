import { isAllowed, setAllowed, getUserInfo } from "@stellar/freighter-api";
import { Contract, networks } from "eras-tour-nft-client";

// Related to Contract
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

async function onSelectSeat() {
  let previous_seat: any;
  let previous_seat_color: any;
  let seats = document.querySelectorAll(`[id^="seat-"]`);
  let seat_info = document.querySelector(".info") as HTMLElement;
  let seat_number_text = document.getElementById("seat");

  console.log("**LOG** seat_info: ", seat_info);

  const ownerOfTx = await erasTourContract.ownerOf({
    seat_num: 3,
  });
  const ownerOfTxResult = await ownerOfTx.simulate();

  console.log("**LOG** ownerOfTxResult: ", ownerOfTxResult.result);

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

  console.log("**LOG** hasContractInit: ", hasContractInit);

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
      console.log("**LOG** hasContractInit: ", hasContractInit);

      console.log("**LOG** seat_number: ", seat_number);
      console.log("**LOG** loggedInUserPubKey: ", loggedInUserPubKey);

      const tx = await erasTourContract.mint({
        to: "GCFAHXAXDMAHPTQ35W42DD2F6LR4E5SQUBEL2RDS5Y4SDRUGMFI4PRV2",
        seat_num: seat_number,
      });

      console.log("**LOG** tx: ", tx);
      const account = await tx.getAccount();
      const pubKey = await tx.getPublicKey();

      console.log("**LOG** account: ", account);
      console.log("**LOG** pubKey: ", pubKey);

      const test = await tx.simulate();
      console.log("**LOG** test: ", test);
      const test2 = await tx.signAndSend();
      console.log("**LOG** test2: ", test2);
    }
  }
}

onSelectSeat();
mintButton.addEventListener("click", mint_seat);
