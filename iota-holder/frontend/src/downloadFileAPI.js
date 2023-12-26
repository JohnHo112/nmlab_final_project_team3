import axios from "axios";

// const instance = axios.create({
//   baseURL: `http://localhost:4000/api`,
// });
const API_ROOT = "http://localhost:4000/api";
const downloadFile = axios.create({
  baseURL: API_ROOT,
  responseType: "blob", // important
});
export default downloadFile;
