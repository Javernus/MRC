import type { Group, Chat } from "../types/types"
import { invoke } from "@tauri-apps/api/tauri"

const getGroups = async (): Promise<Group[]> => {
  return await invoke("get_groups")
}

const getChats = async (groupId: number): Promise<Chat[]> => {
  return await invoke("get_chats", { groupId: groupId })
}

const getLastChat = async (groupId: number): Promise<Chat> => {
  return await invoke("get_newest_chat", { groupId: groupId })
}

const createGroup = async (name: string, bio: string, password: string): Promise<Group> => {
  return invoke("create_group", { name: name, bio: bio, password: password })
}

const removeGroup = async (groupId: number): Promise<void> => {
  invoke("remove_group", { groupId: groupId })
}

const sendChat = async (message: string, groupId: number) => {
  let time = new Date().getTime()
  invoke("send_chat", { message: message, time: time, groupId: groupId })
}

export default {
  getGroups,
  getChats,
  getLastChat,
  createGroup,
  removeGroup,
  sendChat,
}
