import { create, StateCreator } from "zustand";

export interface GlobalStore {}

const useGlobalStore = create<GlobalStore>()((set) => ({}));

export default useGlobalStore;
