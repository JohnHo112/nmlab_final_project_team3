const fs = require("fs");
const removeFileRoute = async (req: any, res: any) => {
  const path = __dirname + "/../../credentials";
  try {
    const files = fs.readdirSync(path);
    const deleteFilePromises = files.map((file:any) =>
      fs.unlinkSync(path + "/" + file)
    );
  } catch (err) {
    console.log(err);
  }
  const path2 = __dirname + "/../../presentations";
  try {
    const files2 = fs.readdirSync(path2);
    const deleteFilePromises2 = files2.map((file:any) =>
      fs.unlinkSync(path2 + "/" + file)
    );
  } catch (err) {
    console.log(err);
  }
};
export default removeFileRoute;
