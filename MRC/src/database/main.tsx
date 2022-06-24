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

const createGroup = async (group: Group): Promise<Group> => {
  const qr = await db.execute("INSERT INTO groups (name, bio) VALUES ($1,$2)", [group.name, group.bio])

  return {
    id: qr.lastInsertId,
    name: group.name,
    bio: group.bio,
  }
}

const removeGroup = async (groupId: number): Promise<void> => {
  await db.execute("DELETE FROM groups WHERE id = $1", [groupId])
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
