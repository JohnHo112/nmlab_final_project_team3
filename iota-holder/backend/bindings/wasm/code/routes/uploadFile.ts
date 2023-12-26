import * as fs from 'fs';
const uploadFileRoute = async (req: any, res: any) => {
  const file = req.files.file
  console.log(file)
  fs.writeFile('./credentials/' + file.name, file.data, (err: any) => {
    if (err) {
      console.error('Error while uploading the file:', err)
    }
  })
  res.send('ok')
}
export default uploadFileRoute
