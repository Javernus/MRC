import type { Group, Chat } from "../types/types"
import { invoke } from "@tauri-apps/api/tauri"

let db = null


const getGroups = async (): Promise<Group[]> => {
  let groups
  invoke("get_groups").then((g: string) => groups = JSON.parse(g))
  console.log(groups)
  return groups
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

const sendChat = async (chat: Chat): Promise<Chat> => {
  const qr = await db.execute("INSERT INTO chats (groupId, name, message) VALUES ($1,$2,$3)", [chat.groupId, chat.name, chat.message])

  console.log("Time: ", (await getChatById(qr.lastInsertId)).time)

  return {
    id: qr.lastInsertId,
    groupId: chat.groupId,
    time: (await getChatById(qr.lastInsertId)).time,
    name: chat.name,
    message: chat.message,
  }
}

export default {
  getGroups,
  getChats,
  createGroup,
  removeGroup,
  sendChat,
}
