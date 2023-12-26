const downloadVPRoute = async (req: any, res: any) => {
  const data = req.query
  res.download(`./presentations/${data.fragment}-presentation.json`)
}
export default downloadVPRoute
