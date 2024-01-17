import { isAllowed, setAllowed, getUserInfo } from "@stellar/freighter-api";

const wrap = document.querySelector("#freighter-wrap");
const ellipsis = document.querySelector("#freighter-wrap .ellipsis");
const button = document.querySelector("[data-connect]");

async function getPk() {
  const { publicKey } = await getUserInfo();
  return publicKey;
}

async function setLoggedIn() {
  const publicKey = await getPk();
  ellipsis.innerHTML = `Signed in as ${publicKey}`;
  ellipsis.title = publicKey;
}

if (await isAllowed()) {
  if (await getPk()) setLoggedIn();
  else wrap.innerHTML = "Freighter is locked.<br>Sign in & refresh the page.";
} else {
  button.addEventListener("click", async () => {
    button.disabled = true;
    await setAllowed();
    await setLoggedIn();
  });
}
