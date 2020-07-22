import * as Comlink from 'comlink';

const onButton = async() => {
  const worker = new Worker("./worker.js", { type: "module"});
  const obj = Comlink.wrap(worker);

  // step1:wasm準備DynamicImport→この後メンバへアクセス可能に。
  await obj.initialize();
  // step2:wasm呼び出す
  const ret = await obj.getNum();
  console.log("ret=" + ret);
};

const ele_btn = document.getElementById("test-button");
ele_btn.onclick = onButton;
