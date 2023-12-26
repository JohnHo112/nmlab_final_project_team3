import axios from "axios";

const baseURL = `http://localhost:8000`;

type VerifyResponse = {
      verified: string
}

const Verify = async (presentation : string, challenge: string) => {
  const { data: { verified } } = await axios.get<VerifyResponse>(`${baseURL}/verify`, {
    params: {
      presentationFile: presentation,
      challenge: challenge,
    },
  });

  return verified;
}

export { Verify }
