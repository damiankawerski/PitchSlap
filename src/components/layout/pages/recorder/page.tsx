import { useEffect, useState } from 'react';
import { Mic, MicOff, FolderOpen, Circle } from 'lucide-react';
import { open } from '@tauri-apps/plugin-dialog';
import { startRecordingInvoke, stopRecordingInvoke, setFileSavePathInvoke, isRecordingInvoke, getFileSavePathInvoke } from '@/lib/invokes/recorder';
import { TransparentCard } from '@/components/ui/transparent-card';
import { Button } from '@/components/ui/button';
import { PageTitle } from '../shared/page-title';
import { Separator } from '@/components/ui/separator';

export default function RecorderPage() {
  const [isRecording, setIsRecording] = useState(false);
  const [savePath, setSavePath] = useState<string | null>(null);

  const handleBrowse = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select output folder',
      });
      if (typeof selected === 'string') {
        setSavePath(selected);
        await setFileSavePathInvoke(selected);
      }
    } catch (e) {
      console.error(e);
    }
  };

  const handleToggleRecording = async () => {
    try {
      if (!isRecording) {
        if (!savePath) {
          return;
        }
        await startRecordingInvoke();
        setIsRecording(true);
      } else {
        await stopRecordingInvoke();
        setIsRecording(false);
      }
    } catch (e) {
      console.error(e);
    }
  };

  useEffect(() => {
    getFileSavePathInvoke().then(setSavePath);
    isRecordingInvoke().then(setIsRecording);
  }, []);

  return (
    <div className="w-full">
      <div className="mx-auto flex w-full max-w-6xl flex-col gap-6 p-6 lg:p-10">
        <header className="flex flex-col gap-2">
          <PageTitle title="Recorder" icon={Mic} />
          <p className="text-sm text-muted-foreground">
            Record audio output to a file on your disk.
          </p>
        </header>

        <Separator />

        <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
          <TransparentCard title="Output Folder" description="Choose where recorded files will be saved.">
            <div className="flex flex-col gap-3">
              <div className="flex items-center gap-3">
                <div className="flex-1 rounded-md border border-white/10 bg-white/5 px-3 py-2 text-sm text-secondary truncate min-w-0">
                  {savePath ?? <span className="text-muted-foreground">No folder selected</span>}
                </div>
                <Button variant="secondary" size="sm" onClick={handleBrowse} className="shrink-0 gap-2">
                  <FolderOpen className="h-4 w-4" />
                  Browse
                </Button>
              </div>
            </div>
          </TransparentCard>


          <TransparentCard title="Recording" description="Start or stop recording audio.">
            <div className="flex flex-col items-center gap-4">
              <button
                onClick={handleToggleRecording}
                className={`relative flex h-20 w-20 items-center justify-center rounded-full border-2 transition-all duration-300 ${
                  isRecording
                    ? 'border-red-500 bg-red-500/20 shadow-[0_0_20px_rgba(239,68,68,0.4)]'
                    : 'border-white/20 bg-white/5 hover:border-white/40 hover:bg-white/10'
                }`}
              >
                {isRecording ? (
                  <MicOff className="h-8 w-8 text-red-400" />
                ) : (
                  <Mic className="h-8 w-8 text-secondary" />
                )}
                {isRecording && (
                  <Circle className="absolute -top-1 -right-1 h-4 w-4 fill-red-500 text-red-500 animate-pulse" />
                )}
              </button>
              <span className="text-sm font-medium text-secondary">
                {isRecording ? 'Recording… click to stop' : 'Click to start recording'}
              </span>
            </div>
          </TransparentCard>
        </div>
      </div>
    </div>
  );
}
