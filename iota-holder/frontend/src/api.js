import axios from "axios";

// const instance = axios.create({
//   baseURL: `http://localhost:4000/api`,
// });
const API_ROOT = "http://localhost:4000/api";
const instance = axios.create({
  baseURL: API_ROOT,
});
export default instance;
