import type { Group, Chat } from "../types/types"
import { invoke } from "@tauri-apps/api/tauri"

let db = null


const getGroups = async (): Promise<string> => {
  return invoke("get_groups")
}

const getChatById = async (id: number): Promise<Chat> => {
  return await db.select("SELECT * FROM chats WHERE id = ?", [id])[0]
}

const getChats = async (): Promise<Chat[]> => {
  return await db.select("SELECT * FROM chats")
}

const createGroup = async (name: string, bio: string, password: string): Promise<Group> => {
  invoke("create_group", { name: name, bio: bio, password: password })
  return { name: name, bio: bio, id: 20 }
}

const removeGroup = async (groupId: number): Promise<void> => {
  invoke("remove_group", { groupId: groupId })
}

const sendChat = async (message: string, groupId: number): Promise<Chat> => {
  invoke("send_chat", { message: message, groupId: groupId })

  return {
    id: 0,
    groupId: groupId,
    time: (new Date()).getTime(),
    name: "Jake",
    message: message,
  }
}

export default {
  getGroups,
  getChats,
  createGroup,
  removeGroup,
  sendChat,
}
