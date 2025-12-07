import { ref } from 'vue'

export function useDrumRoll() {
    const audioContext = ref<AudioContext | null>(null)
    const drumBuffer = ref<AudioBuffer | null>(null)
    const tympanyBuffer = ref<AudioBuffer | null>(null)
    const cymbalBuffer = ref<AudioBuffer | null>(null)
    const sourceNode = ref<AudioBufferSourceNode | null>(null)
    const isPlaying = ref(false)

    const initAudio = async () => {
        if (!audioContext.value) {
            audioContext.value = new (window.AudioContext || (window as any).webkitAudioContext)()
        }

        if (!drumBuffer.value) {
            try {
                const response = await fetch('/drumroll.mp3')
                const arrayBuffer = await response.arrayBuffer()
                if (audioContext.value) {
                    drumBuffer.value = await audioContext.value.decodeAudioData(arrayBuffer)
                }
            } catch (e) {
                console.error('Failed to load drumroll.mp3', e)
            }
        }

        if (!tympanyBuffer.value) {
            try {
                const response = await fetch('/timpaniroll.mp3')
                const arrayBuffer = await response.arrayBuffer()
                if (audioContext.value) {
                    tympanyBuffer.value = await audioContext.value.decodeAudioData(arrayBuffer)
                }
            } catch (e) {
                console.error('Failed to load timpaniroll.mp3', e)
            }
        }

        if (!cymbalBuffer.value) {
            try {
                const response = await fetch('/cymbal.mp3')
                const arrayBuffer = await response.arrayBuffer()
                if (audioContext.value) {
                    cymbalBuffer.value = await audioContext.value.decodeAudioData(arrayBuffer)
                }
            } catch (e) {
                console.error('Failed to load cymbal.mp3', e)
            }
        }
    }

    const play = async (isLast: boolean = false) => {
        await initAudio()
        const buffer = isLast ? tympanyBuffer.value : drumBuffer.value
        if (!audioContext.value || !buffer) return
        if (isPlaying.value) return

        if (audioContext.value.state === 'suspended') {
            await audioContext.value.resume()
        }

        const source = audioContext.value.createBufferSource()
        source.buffer = buffer
        source.loop = true

        const gain = audioContext.value.createGain()
        gain.gain.value = isLast ? 1.5 : 0.8

        source.connect(gain)
        gain.connect(audioContext.value.destination)

        source.start()
        sourceNode.value = source
        isPlaying.value = true
    }

    const stop = (delay: number = 0) => {
        if (sourceNode.value && audioContext.value) {
            sourceNode.value.stop(audioContext.value.currentTime + delay)
            setTimeout(() => {
                sourceNode.value = null
                isPlaying.value = false
            }, delay * 1000)
        }
    }

    const playCymbal = async (delay: number = 0) => {
        await initAudio()
        if (!audioContext.value || !cymbalBuffer.value) return

        const source = audioContext.value.createBufferSource()
        source.buffer = cymbalBuffer.value

        const gain = audioContext.value.createGain()
        gain.gain.value = 0.8

        source.connect(gain)
        gain.connect(audioContext.value.destination)

        source.start(audioContext.value.currentTime + delay)
    }

    return {
        play,
        stop,
        playCymbal
    }
}
