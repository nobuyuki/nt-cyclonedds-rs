/// Representing a Topic
///
pub struct Topic {}

//class Topic(Entity, Generic[_S]):

impl Topic {
    pub fn new() -> Self {
        panic!()

        /*
            def __init__(
            self,
            domain_participant: 'cyclonedds.domain.DomainParticipant',
            topic_name: AnyStr,
            data_type: Type[_S],
            qos: Optional[Qos] = None,
            listener: Optional[Listener] = None):
        if qos is not None:
            if isinstance(qos, LimitedScopeQos) and not isinstance(qos, TopicQos):
                raise TypeError(f"{qos} is not appropriate for a Topic")
            elif not isinstance(qos, Qos):
                raise TypeError(f"{qos} is not a valid qos object")

        if listener is not None:
            if not isinstance(listener, Listener):
                raise TypeError(f"{listener} is not a valid listener object.")

        if not hasattr(data_type, "__idl__"):
            raise TypeError(f"{data_type} is not an idl type.")

        self.data_type = data_type
        data_type.__idl__.populate()
        data_type.__idl__.fill_type_data()

        cqos = _CQos.qos_to_cqos(qos) if qos else None
        try:
            super().__init__(
                ddspy_topic_create(
                    domain_participant._ref,
                    topic_name,
                    data_type,
                    cqos,
                    listener._ref if listener else None
                ),
                listener=listener
            )
        finally:
            if cqos:
                _CQos.cqos_destroy(cqos)

        self._keepalive_entities = [self.participant]

        */
    }

    pub fn get_name(&self) -> String {
        panic!()
        /*
            def get_name(self, max_size=256) -> str:
                name = (ct.c_char * max_size)()
                name_pt = ct.cast(name, ct.c_char_p)
                ret = self._get_name(self._ref, name_pt, max_size)
                if ret < 0:
                    raise DDSException(ret, f"Occurred while fetching a topic name for {repr(self)}")
                return bytes(name).split(b'\0', 1)[0].decode("ASCII")
        */
    }

    pub fn get_type_name(&self) -> String {
        panic!()
        /*

        def get_type_name(self, max_size=256) -> str:
            name = (ct.c_char * max_size)()
            name_pt = ct.cast(name, ct.c_char_p)
            ret = self._get_type_name(self._ref, name_pt, max_size)
            if ret < 0:
                raise DDSException(ret, f"Occurred while fetching a topic type name for {repr(self)}")
            return bytes(name).split(b'\0', 1)[0].decode("ASCII")

        */
    }
}
