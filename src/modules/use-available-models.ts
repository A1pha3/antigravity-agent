import {create} from "zustand";
import type {AntigravityAccount} from "@/commands/types/account.types.ts";
import {CloudCodeAPI} from "@/services/cloudcode-api.ts";
import {CloudCodeAPITypes} from "@/services/cloudcode-api.types.ts";

type State = {
  data: Record<string, CloudCodeAPITypes.FetchAvailableModelsResponse>
}

type Actions = {
  fetchData: (antigravityAccount: AntigravityAccount) => Promise<void>
}

export const useAvailableModels = create<State & Actions>((setState, getState) => ({
  data: {},
  fetchData: async (antigravityAccount: AntigravityAccount) => {
    const codeAssistResponse = await CloudCodeAPI.loadCodeAssist(antigravityAccount.api_key);

    const modelsResponse = await CloudCodeAPI.fetchAvailableModels(antigravityAccount.api_key, codeAssistResponse.cloudaicompanionProject);

    setState({
      data: {
        ...getState().data,
        [antigravityAccount.api_key]: modelsResponse
      }
    })
  }
}))
