import { ref } from 'vue';
import { defineStore } from 'pinia';

export interface Chat {
  id: string;
  name: string;
  roles: string[];
  avatar?: string;
  permissions?: Record<string, boolean>;
}

export const useChatsStore = defineStore('chat-store', () => {
  const chats = ref<Chat[]>([
    {
      id: '9ccd2ae4-f803-400d-be21-0967d6615636',
      name: 'Work',
      roles: ['admin', 'member', 'guest'],
    },
  ]);

  const getChatById = (id: string) => chats.value.find((chat) => chat.id === id) || null;

  return {
    chats,
    getChatById,
  };
});
