import { Component, createSignal, createResource, For } from 'solid-js'
import './App.scss'
import './colours.scss'
import { Button, ChatItem, GroupItem, HamburgerX, Panel, InputField, Settings, Terminal } from './components'
import type { Group, Chat } from './types/types'
import DB from './database/main'
import cl from 'clsx'

let activeGroups = [1]

// Return the chat message with the latest timestamp.
const lastChat = async (groupId: number) => {
  const chat = (await DB.getLastChat(groupId))
  if (chat["group_id"] === 0) return null
  return "<" + chat["name"] + "> " + chat["message"]
}

const chatsFromGroup = async (group: Group) => {
  return await DB.getChats(group.id)
}

const requestGroups = async (setGroups) => {
  const groups: Group[] = await DB.getGroups()
  setGroups(groups)
}

const App: Component = () => {
  let [plusMenu, setPlusMenu] = createSignal(false)
  let [groups, setGroups] = createSignal([])
  let [showCreateGroup, setShowCreateGroup] = createSignal(false)
  let [search, setSearch] = createSignal('')
  let [openGroup, setOpenGroup] = createSignal(null)
  let [showGroupInfo, setShowGroupInfo] = createSignal(false)
  let [chats, chatRefetch] = createResource(openGroup, chatsFromGroup)
  let [groupName, setGroupName] = createSignal('')
  let [groupBio, setGroupBio] = createSignal('')
  let [groupPassword, setGroupPassword] = createSignal('')

  requestGroups(setGroups)

  const searchGroups = (event) => {
    setSearch(event.target.value)
  }

  const createGroup = async () => {
    const group = await DB.createGroup(groupName(), groupBio(), groupPassword())
    setGroups([...groups(), group])

    setGroupName('')
    setGroupBio('')
    setGroupPassword('')

    setShowCreateGroup(false)
    setPlusMenu(false)
    setOpenGroup(group)
  }

  const deleteGroup = async () => {
    setGroups(groups().filter(group => group.id !== openGroup().id))
    await DB.removeGroup(openGroup().id)
    setOpenGroup(null)
  }

  // Send the message using DB and add it to the chats array.
  const sendChat = async (message: string, groupId: number) => {
    await DB.sendChat(message, groupId)
    chatRefetch.refetch()
  }

  return (
    <>
      <Panel visible>
        <div class='top-bar'>
          <HamburgerX
            size='2rem'
            onclick={() => {
                if (showCreateGroup()) {
                  setPlusMenu(false)
                  setShowCreateGroup(false)
                } else {
                  setPlusMenu(!plusMenu())
                }
              }
            }
          />
          <InputField placeholder="Search" oninput={searchGroups} />
        </div>
        <div class={cl('create-group', { 'create-group--visible': showCreateGroup() })}>
          <p class='create-group__header'>Create Group</p>
          <InputField placeholder="Group Name" oninput={(event) => setGroupName(event.target.value)} value={groupName()} />
          <InputField placeholder="Bio" oninput={(event) => setGroupBio(event.target.value)} value={groupBio()} />
          <InputField placeholder="Password" type="password" oninput={(event) => setGroupPassword(event.target.value)} value={groupPassword()} />
          <Button onclick={createGroup} type="submit">Create</Button>
        </div>
        <div class={cl('plus-menu', { 'plus-menu--visible': plusMenu() })}>
          <GroupItem
            name='Create Group'
            onclick={() => setShowCreateGroup(true)}
          />
          <GroupItem
            name='Join Group'
            onclick={() => {}}
          />
        </div>
        <div class='groups'>
          <For each={groups().filter(group => group.name.toLowerCase().includes(search().toLowerCase()))}>{(group: Group) =>
            <GroupItem
              name={group.name}
              lastChat={lastChat}
              status={activeGroups.includes(group.id) ? 'green' : 'yellow'}
              active={group === openGroup()}
              groupId={group.id}
              onclick={() => setOpenGroup(group)}
            />
          }</For>
        </div>
      </Panel>

      <Terminal
        disabled={!openGroup()}
        send={(message) => {sendChat(message, openGroup().id)}}
      >
        <For each={chats()}>{(chat) =>
          <ChatItem
            chat={chat}
          ></ChatItem>
        }</For>
      </Terminal>
      {
        !!openGroup() &&
        <Panel right fitContent visible={showGroupInfo()}>
          <Settings size='2rem' onclick={() => setShowGroupInfo(!showGroupInfo())} />
          <div class='top-bar'>
            <div class='group-icon'>
              <div class='group-icon__indicator group-icon__indicator--green' />
            </div>
            <p class='group-name'>{openGroup().name}</p>
          </div>
          {
            openGroup().bio &&
            <div class='bio'>
              <p>{openGroup().bio}</p>
            </div>
          }
          <div class='delete-group' onclick={deleteGroup}>
            <p>Delete Group</p>
          </div>
        </Panel>
      }
    </>
  )
}

export default App
