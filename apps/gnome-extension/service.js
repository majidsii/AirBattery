export const BUS_NAME = 'io.github.airbattery.Service';
export const OBJECT_PATH = '/io/github/airbattery/Service';
export const INTERFACE_NAME = 'io.github.airbattery.Service1';

export const SERVICE_XML = `
<node>
  <interface name="io.github.airbattery.Service1">
    <method name="GetSnapshot">
      <arg name="snapshot_json" type="s" direction="out"/>
    </method>
    <method name="Refresh">
      <arg name="snapshot_json" type="s" direction="out"/>
    </method>
    <method name="OpenSettings"/>
    <method name="ShowMainWindow"/>
    <signal name="SnapshotChanged">
      <arg name="snapshot_json" type="s"/>
    </signal>
  </interface>
</node>`;
